use super::desktop::{Desktop, Preferences};
use crate::infrastructure::logging::LogBuffer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    io::Read,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tauri::Manager;

const TOOL_NAMES: [&str; 6] = [
    "get_library_overview",
    "search_library",
    "create_playlist_with_tracks",
    "play_tracks",
    "control_playback",
    "set_appearance",
];

fn endpoint(base: &str) -> Result<String, String> {
    let base = base.trim();
    if base.len() > 2048 || base.chars().any(char::is_control) {
        return Err("AI_URL_INVALID".into());
    }
    let parsed = ureq::get(base)
        .request_url()
        .map_err(|_| "AI_URL_INVALID")?;
    let url = parsed.as_url();
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("AI_URL_INVALID：仅支持不含凭证、查询参数或片段的 HTTP(S) Base URL".into());
    }
    let mut target = url.clone();
    target.set_path(&format!(
        "{}/chat/completions",
        url.path().trim_end_matches('/')
    ));
    Ok(target.into())
}

#[derive(Default)]
pub(crate) struct ConfigPatch {
    pub enabled: Option<bool>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
}
pub(crate) fn apply_config(
    current: &Preferences,
    patch: ConfigPatch,
) -> Result<Preferences, String> {
    let mut next = current.clone();
    if let Some(enabled) = patch.enabled {
        next.ai_enabled = enabled;
    }
    if let Some(base) = patch.base_url {
        let base = base.trim();
        endpoint(base)?;
        next.ai_base_url = Some(base.trim_end_matches('/').to_string());
    }
    if let Some(model) = patch.model {
        let model = model.trim();
        if model.is_empty() || model.len() > 128 || model.chars().any(char::is_control) {
            return Err("AI_MODEL_INVALID".into());
        }
        next.ai_model = Some(model.into());
    }
    if let Some(key) = patch.api_key {
        let key = key.trim();
        if key.len() > 512 || !key.bytes().all(|b| b.is_ascii_graphic()) {
            return Err("AI_KEY_INVALID".into());
        }
        next.ai_api_key = (!key.is_empty()).then(|| key.into());
        if key.is_empty() {
            next.ai_enabled = false;
        }
    }
    Ok(next)
}
#[derive(Serialize)]
pub struct Config {
    enabled: bool,
    base_url: String,
    model: String,
    has_key: bool,
    revision: u64,
}
impl Config {
    fn from_preferences(p: &Preferences, revision: u64) -> Self {
        Self {
            enabled: p.ai_enabled,
            base_url: p
                .ai_base_url
                .clone()
                .unwrap_or_else(|| "https://api.openai.com/v1".into()),
            model: p.ai_model.clone().unwrap_or_else(|| "gpt-4o-mini".into()),
            has_key: p.ai_api_key.as_ref().is_some_and(|k| !k.is_empty()),
            revision,
        }
    }
}
pub struct Agent {
    client: ureq::Agent,
    configuration: Mutex<()>,
    revision: AtomicU64,
    in_flight: AtomicBool,
}
impl Agent {
    pub fn new() -> Self {
        Self {
            client: ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(30))
                .redirects(0)
                .user_agent("MusicMine/0.1.0")
                .build(),
            configuration: Mutex::new(()),
            revision: AtomicU64::new(0),
            in_flight: AtomicBool::new(false),
        }
    }
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AiFunction {
    pub name: String,
    pub arguments: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AiToolCall {
    pub id: String,
    pub r#type: String,
    pub function: AiFunction,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AiMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<AiToolCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}
#[derive(Serialize)]
pub struct AiTurnResponse {
    message: AiMessage,
    finish_reason: String,
}

fn valid_call(call: &AiToolCall) -> bool {
    !call.id.is_empty()
        && call.id.len() <= 128
        && !call.id.chars().any(char::is_control)
        && call.r#type == "function"
        && !call.function.name.is_empty()
        && call.function.name.len() <= 64
        && call
            .function
            .name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && call.function.arguments.len() <= 8192
}
fn validate_input(messages: &[AiMessage], tools: &[Value]) -> Result<(), String> {
    if messages.is_empty() || messages.len() > 100 || tools.len() > 6 {
        return Err("AI_INPUT_LIMIT".into());
    }
    let mut pending = std::collections::HashSet::new();
    for m in messages {
        if !matches!(m.role.as_str(), "system" | "user" | "assistant" | "tool")
            || m.content.as_ref().is_some_and(|s| s.len() > 64_000)
            || m.tool_calls.len() > 8
            || m.tool_calls.iter().any(|c| !valid_call(c))
        {
            return Err("AI_MESSAGE_INVALID".into());
        }
        if m.role == "tool" {
            if m.content.is_none()
                || !m.tool_calls.is_empty()
                || !m.tool_call_id.as_ref().is_some_and(|id| pending.remove(id))
            {
                return Err("AI_TOOL_HISTORY_INVALID".into());
            }
        } else {
            if !pending.is_empty()
                || m.tool_call_id.is_some()
                || (m.role != "assistant" && (m.content.is_none() || !m.tool_calls.is_empty()))
                || (m.content.is_none() && m.tool_calls.is_empty())
            {
                return Err("AI_MESSAGE_INVALID".into());
            }
            for c in &m.tool_calls {
                if !pending.insert(c.id.clone()) {
                    return Err("AI_TOOL_HISTORY_INVALID".into());
                }
            }
        }
    }
    if !pending.is_empty() {
        return Err("AI_TOOL_HISTORY_INVALID".into());
    }
    let mut names = std::collections::HashSet::new();
    for tool in tools {
        let name = tool
            .pointer("/function/name")
            .and_then(Value::as_str)
            .ok_or("AI_TOOL_INVALID")?;
        if tool.get("type").and_then(Value::as_str) != Some("function")
            || !TOOL_NAMES.contains(&name)
            || !names.insert(name)
            || tool
                .pointer("/function/parameters/type")
                .and_then(Value::as_str)
                != Some("object")
        {
            return Err("AI_TOOL_INVALID".into());
        }
    }
    if serde_json::to_vec(&(messages, tools))
        .map_err(|_| "AI_INPUT_INVALID")?
        .len()
        > 256_000
    {
        return Err("AI_INPUT_LIMIT".into());
    }
    Ok(())
}
fn decode_response(response: ureq::Response, secret: &str) -> Result<AiTurnResponse, String> {
    let status = response.status();
    if !(200..300).contains(&status) {
        return Err(format!("AI_HTTP_{status}"));
    }
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(1_000_001)
        .read_to_end(&mut bytes)
        .map_err(|_| "AI_BODY_READ")?;
    if bytes.len() > 1_000_000 {
        return Err("AI_BODY_LIMIT".into());
    }
    let value: Value = serde_json::from_slice(&bytes).map_err(|_| "AI_RESPONSE_INVALID")?;
    let choice = value.pointer("/choices/0").ok_or("AI_RESPONSE_INVALID")?;
    // Select known fields only: provider extensions never become executable instructions.
    let m = choice.get("message").ok_or("AI_RESPONSE_INVALID")?;
    if m.get("role").and_then(Value::as_str) != Some("assistant") {
        return Err("AI_RESPONSE_INVALID".into());
    }
    let content = m
        .get("content")
        .and_then(Value::as_str)
        .or_else(|| m.get("refusal").and_then(Value::as_str))
        .map(|s| {
            if secret.is_empty() {
                s.to_string()
            } else {
                s.replace(secret, "***")
            }
        });
    let calls: Vec<AiToolCall> = match m.get("tool_calls").filter(|v| !v.is_null()) {
        Some(v) => serde_json::from_value(v.clone()).map_err(|_| "AI_TOOL_CALL_INVALID")?,
        None => Vec::new(),
    };
    if calls.len() > 8
        || calls.iter().any(|c| !valid_call(c))
        || (content.is_none() && calls.is_empty())
    {
        return Err("AI_RESPONSE_INVALID".into());
    }
    if !secret.is_empty()
        && calls.iter().any(|c| {
            c.id.contains(secret)
                || c.function.name.contains(secret)
                || c.function.arguments.contains(secret)
        })
    {
        return Err("AI_RESPONSE_SECRET".into());
    }
    Ok(AiTurnResponse {
        message: AiMessage {
            role: "assistant".into(),
            content,
            tool_calls: calls,
            tool_call_id: None,
        },
        finish_reason: choice
            .get("finish_reason")
            .and_then(Value::as_str)
            .filter(|s| {
                matches!(
                    *s,
                    "stop" | "tool_calls" | "length" | "content_filter" | "function_call"
                )
            })
            .unwrap_or("stop")
            .chars()
            .take(64)
            .collect(),
    })
}
fn log(app: &tauri::AppHandle, summary: &str, key: Option<&str>) {
    app.state::<Arc<LogBuffer>>()
        .push_redacted(&format!("[ai] {summary}"), key);
}

#[tauri::command]
pub async fn ai_config(
    app: tauri::AppHandle,
    enabled: Option<bool>,
    base_url: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<Config, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let agent = app.state::<Agent>();
        let _guard = agent.configuration.lock().map_err(|_| "AI_CONFIG_LOCK")?;
        let desktop = app.state::<Desktop>();
        let p = if enabled.is_some() || base_url.is_some() || api_key.is_some() || model.is_some() {
            let p = desktop.configure_ai(ConfigPatch {
                enabled,
                base_url,
                api_key,
                model,
            })?;
            agent.revision.fetch_add(1, Ordering::SeqCst);
            log(&app, "config saved key=***", p.ai_api_key.as_deref());
            p
        } else {
            desktop.scrobble_preferences()?
        };
        Ok(Config::from_preferences(
            &p,
            agent.revision.load(Ordering::SeqCst),
        ))
    })
    .await
    .map_err(|_| "AI_CONFIG_TASK".to_string())?
}
struct InFlight<'a>(&'a AtomicBool);
impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}
#[tauri::command]
pub async fn ai_chat_turn(
    app: tauri::AppHandle,
    messages: Vec<AiMessage>,
    tools: Vec<Value>,
) -> Result<AiTurnResponse, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let agent = app.state::<Agent>();
        agent
            .in_flight
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .map_err(|_| "AI_BUSY")?;
        let _busy = InFlight(&agent.in_flight);
        let (p, revision) = {
            let _guard = agent.configuration.lock().map_err(|_| "AI_CONFIG_LOCK")?;
            (
                app.state::<Desktop>().scrobble_preferences()?,
                agent.revision.load(Ordering::SeqCst),
            )
        };
        if !p.ai_enabled {
            return Err("AI_DISABLED：请前往设置开启 AI 助手".into());
        }
        let key = p
            .ai_api_key
            .as_deref()
            .filter(|s| !s.is_empty())
            .ok_or("AI_KEY_MISSING：请前往设置配置 API Key")?;
        let result: Result<AiTurnResponse, String> = (|| {
            validate_input(&messages, &tools)?;
            let config = Config::from_preferences(&p, revision);
            let url = endpoint(&config.base_url)?;
            log(
                &app,
                &format!(
                    "request messages={} tools={} tool_results={}",
                    messages.len(),
                    tools.len(),
                    messages.iter().filter(|m| m.role == "tool").count()
                ),
                Some(key),
            );
            // No configuration or Controller lock is held during I/O. Do not retry mutations.
            let mut payload = json!({"model":config.model,"messages":messages,"stream":false});
            if !tools.is_empty() {
                payload["tools"] = json!(tools);
                payload["tool_choice"] = json!("auto");
            }
            let response = agent
                .client
                .post(&url)
                .set("Authorization", &format!("Bearer {key}"))
                .send_json(payload);
            let response = match response {
                Ok(r) | Err(ureq::Error::Status(_, r)) => r,
                Err(_) => return Err("AI_NETWORK".into()),
            };
            if agent.revision.load(Ordering::SeqCst) != revision {
                return Err("AI_CONFIG_CHANGED".into());
            }
            let turn = decode_response(response, key)?;
            if agent.revision.load(Ordering::SeqCst) != revision {
                return Err("AI_CONFIG_CHANGED".into());
            }
            for call in &turn.message.tool_calls {
                log(
                    &app,
                    &format!(
                        "tool requested {}",
                        if TOOL_NAMES.contains(&call.function.name.as_str()) {
                            call.function.name.as_str()
                        } else {
                            "unknown"
                        }
                    ),
                    Some(key),
                );
            }
            log(
                &app,
                &format!("response tools={}", turn.message.tool_calls.len()),
                Some(key),
            );
            Ok(turn)
        })();
        if let Err(code) = &result {
            log(&app, code, Some(key));
        }
        result
    })
    .await
    .map_err(|_| "AI_CHAT_TASK".to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(role: &str, content: &str) -> AiMessage {
        AiMessage {
            role: role.into(),
            content: Some(content.into()),
            tool_calls: vec![],
            tool_call_id: None,
        }
    }
    #[test]
    fn public_configuration_and_errors_do_not_expose_keys_or_provider_bodies() {
        let mut p = Preferences::default();
        p.ai_api_key = Some("fixture-secret".into());
        let public = serde_json::to_string(&Config::from_preferences(&p, 1)).unwrap();
        assert!(!public.contains("fixture-secret"));
        assert!(!public.contains("ai_api_key"));
        assert!(public.contains("\"has_key\":true"));
        assert_eq!(
            decode_response(
                ureq::Response::new(401, "Unauthorized", "fixture-secret private-response")
                    .unwrap(),
                "fixture-secret"
            )
            .err()
            .as_deref(),
            Some("AI_HTTP_401")
        );
        let response=decode_response(ureq::Response::new(200,"OK",r#"{"choices":[{"message":{"role":"assistant","content":"fixture-secret"},"finish_reason":"stop"}]}"#).unwrap(),"fixture-secret").unwrap();
        assert_eq!(response.message.content.as_deref(), Some("***"));
        assert!(apply_config(
            &p,
            ConfigPatch {
                api_key: Some("bad\nheader".into()),
                ..Default::default()
            }
        )
        .is_err());
    }
    #[test]
    fn messages_enforce_bounds_and_complete_tool_exchanges() {
        assert!(validate_input(&[message("user", "hi")], &[]).is_ok());
        assert!(validate_input(&[message("shell", "erase")], &[]).is_err());
        assert!(validate_input(&[], &[]).is_err());
        assert!(validate_input(&vec![message("user", "a"); 101], &[]).is_err());
        let call = AiToolCall {
            id: "call_1".into(),
            r#type: "function".into(),
            function: AiFunction {
                name: "control_playback".into(),
                arguments: "{}".into(),
            },
        };
        let assistant = AiMessage {
            role: "assistant".into(),
            content: None,
            tool_calls: vec![call],
            tool_call_id: None,
        };
        assert!(validate_input(std::slice::from_ref(&assistant), &[]).is_err());
        let result = AiMessage {
            role: "tool".into(),
            content: Some("{\"ok\":true}".into()),
            tool_calls: vec![],
            tool_call_id: Some("call_1".into()),
        };
        assert!(validate_input(&[assistant, result], &[]).is_ok());
        assert!(validate_input(&[message("user","hi")],&[json!({"type":"function","function":{"name":"run_shell","parameters":{"type":"object"}}})]).is_err());
    }
    #[test]
    fn invalid_or_oversized_responses_never_become_tool_calls() {
        assert!(decode_response(ureq::Response::new(200, "OK", "not json").unwrap(), "").is_err());
        assert_eq!(
            decode_response(
                ureq::Response::new(200, "OK", &"x".repeat(1_000_001)).unwrap(),
                ""
            )
            .err()
            .as_deref(),
            Some("AI_BODY_LIMIT")
        );
        let body=json!({"choices":[{"message":{"role":"assistant","content":null,"tool_calls":[{"id":"call_1","type":"function","function":{"name":"control_playback","arguments":"fixture-secret"}}]}}]}).to_string();
        assert_eq!(
            decode_response(
                ureq::Response::new(200, "OK", &body).unwrap(),
                "fixture-secret"
            )
            .err()
            .as_deref(),
            Some("AI_RESPONSE_SECRET")
        );
    }

    #[test]
    fn url_validation_allows_http_services_and_blocks_other_schemes_or_embedded_credentials() {
        assert_eq!(
            endpoint("https://api.openai.com/v1/").unwrap(),
            "https://api.openai.com/v1/chat/completions"
        );
        assert_eq!(
            endpoint("http://localhost:11434/v1").unwrap(),
            "http://localhost:11434/v1/chat/completions"
        );
        for bad in [
            "file:///C:/secret",
            "javascript:alert(1)",
            "ftp://example.com",
            "https://key@example.com/v1",
            "https://example.com/v1?api_key=secret",
            "https://example.com/#fragment",
            "http://",
            "",
        ] {
            assert!(endpoint(bad).is_err(), "accepted invalid endpoint");
        }
    }

    #[test]
    fn old_preferences_default_ai_off_and_have_usable_provider_defaults() {
        let p: crate::platform::desktop::Preferences = serde_json::from_str("{}").unwrap();
        let json = serde_json::to_value(p).unwrap();
        assert_eq!(json["ai_enabled"], false);
        assert_eq!(json["ai_base_url"], "https://api.openai.com/v1");
        assert_eq!(json["ai_model"], "gpt-4o-mini");
    }
}
