//! Fixed-provider, bounded HTTP for explicit metadata searches. No raw upstream errors escape.
use crate::catalog::enrichment::{valid_mbid, MAX_IMAGE};
use serde_json::Value;
use std::io::Read;
use std::sync::Mutex;
use std::time::{Duration, Instant};

static MB_LAST: Mutex<Option<Instant>> = Mutex::new(None);
static LRC_LAST: Mutex<Option<Instant>> = Mutex::new(None);
static ITUNES_LAST: Mutex<Option<Instant>> = Mutex::new(None);
pub struct Client {
    agent: ureq::Agent,
}
impl Client {
    pub fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(10))
                .redirects(0)
                .user_agent("MusicMine/0.1.0")
                .build(),
        }
    }
    fn json(&self, request: ureq::Request, code: &str) -> Result<Value, String> {
        let response = request.call().map_err(|_| code.to_string())?;
        let response_code = code.replace("_NETWORK", "_RESPONSE");
        let bytes = bounded(response.into_reader(), 2 * 1024 * 1024).map_err(|error| {
            if error == "METADATA_IMAGE_SIZE" {
                response_code.clone()
            } else {
                code.to_string()
            }
        })?;
        serde_json::from_slice(&bytes).map_err(|_| response_code)
    }
    pub fn recordings(&self, title: &str, artist: &str) -> Result<Value, String> {
        // Serialize only MusicBrainz requests, never hold Catalog/Controller locks over HTTP.
        let mut last = MB_LAST.lock().map_err(|_| "METADATA_MB_BUSY".to_string())?;
        if let Some(t) = *last {
            std::thread::sleep(Duration::from_millis(1100).saturating_sub(t.elapsed()));
        }
        *last = Some(Instant::now());
        let escaped = |s: &str| {
            s.chars()
                .flat_map(|c| {
                    if "+-&|!(){}[]^\"~*?:\\/".contains(c) {
                        vec!['\\', c]
                    } else {
                        vec![c]
                    }
                })
                .collect::<String>()
        };
        let mut query = format!("recording:\"{}\"", escaped(title));
        if !artist.is_empty() {
            query.push_str(&format!(" AND artist:\"{}\"", escaped(artist)));
        }
        self.json(
            self.agent
                .get("https://musicbrainz.org/ws/2/recording")
                .query("query", &query)
                .query("fmt", "json")
                .query("limit", "5"),
            "METADATA_MB_NETWORK",
        )
    }
    pub fn lyrics(&self, title: &str, artist: &str) -> Result<Value, String> {
        let mut last = LRC_LAST
            .lock()
            .map_err(|_| "METADATA_LRC_BUSY".to_string())?;
        if let Some(t) = *last {
            std::thread::sleep(Duration::from_millis(300).saturating_sub(t.elapsed()));
        }
        *last = Some(Instant::now());
        let mut request = self
            .agent
            .get("https://lrclib.net/api/search")
            .query("track_name", title);
        if !artist.is_empty() {
            request = request.query("artist_name", artist);
        }
        self.json(request, "METADATA_LRC_NETWORK")
    }
    pub fn itunes(&self, title: &str, artist: &str) -> Result<Value, String> {
        country_fallback(|country| {
            let mut last = ITUNES_LAST
                .lock()
                .map_err(|_| "METADATA_ITUNES_NETWORK".to_string())?;
            if let Some(t) = *last {
                std::thread::sleep(Duration::from_millis(3100).saturating_sub(t.elapsed()));
            }
            *last = Some(Instant::now());
            let query = format!("{title} {artist}");
            self.json(
                self.agent
                    .get("https://itunes.apple.com/search")
                    .timeout(Duration::from_secs(8))
                    .query("term", query.trim())
                    .query("media", "music")
                    .query("entity", "song")
                    .query("limit", "5")
                    .query("country", country),
                "METADATA_ITUNES_NETWORK",
            )
        })
    }
    pub fn apple_cover(&self, url: &str) -> Result<Vec<u8>, String> {
        self.download(
            url.to_string(),
            validate_itunes_cover_url,
            Duration::from_secs(8),
        )
    }
    pub fn cover(&self, mbid: &str, preview: bool) -> Result<Vec<u8>, String> {
        if !valid_mbid(mbid) {
            return Err("METADATA_COVER_INVALID".into());
        }
        let url = format!(
            "https://coverartarchive.org/release/{mbid}/front{}",
            if preview { "-250" } else { "" }
        );
        self.download(url, validate_cover_url, Duration::from_secs(10))
    }
    fn download(
        &self,
        mut url: String,
        validate: fn(&str) -> Result<(), String>,
        timeout: Duration,
    ) -> Result<Vec<u8>, String> {
        let deadline = Instant::now() + timeout;
        for _ in 0..5 {
            validate(&url)?;
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err("METADATA_COVER_NETWORK".into());
            }
            let response = self
                .agent
                .get(&url)
                .timeout(remaining)
                .call()
                .map_err(|_| "METADATA_COVER_NETWORK".to_string())?;
            if matches!(response.status(), 301 | 302 | 303 | 307 | 308) {
                let location = response.header("Location").ok_or("METADATA_COVER_URL")?;
                let parsed = ureq::get(&url)
                    .request_url()
                    .map_err(|_| "METADATA_COVER_URL".to_string())?;
                url = parsed
                    .as_url()
                    .join(location)
                    .map_err(|_| "METADATA_COVER_URL".to_string())?
                    .to_string();
                continue;
            }
            if response
                .header("Content-Length")
                .and_then(|s| s.parse::<usize>().ok())
                .is_some_and(|n| n > MAX_IMAGE)
            {
                return Err("METADATA_IMAGE_SIZE".into());
            }
            return bounded(response.into_reader(), MAX_IMAGE);
        }
        Err("METADATA_COVER_REDIRECT".into())
    }
}
pub fn validate_itunes_cover_url(raw: &str) -> Result<(), String> {
    if raw.len() > 2048 || raw.chars().any(char::is_control) {
        return Err("METADATA_COVER_URL".into());
    }
    let parsed = ureq::get(raw)
        .request_url()
        .map_err(|_| "METADATA_COVER_URL")?;
    let url = parsed.as_url();
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some_and(|p| p != 443)
        || url.query().is_some()
        || url.fragment().is_some()
        || !url.host_str().is_some_and(|h| h.ends_with(".mzstatic.com"))
        || !url.path().starts_with("/image/thumb/")
        || !url.path().ends_with("/600x600bb.jpg")
    {
        return Err("METADATA_COVER_URL".into());
    }
    Ok(())
}
fn country_fallback(mut fetch: impl FnMut(&str) -> Result<Value, String>) -> Result<Value, String> {
    let mut empty = None;
    let mut response_error = None;
    for country in ["cn", "us"] {
        match fetch(country) {
            Ok(value) => match value["results"].as_array() {
                Some(results) if !results.is_empty() => return Ok(value),
                Some(_) => empty = Some(value),
                None => response_error = Some("METADATA_ITUNES_RESPONSE".to_string()),
            },
            Err(error) if error == "METADATA_ITUNES_NETWORK" => {}
            Err(error) => response_error = Some(error),
        }
    }
    empty.ok_or_else(|| response_error.unwrap_or_else(|| "METADATA_ITUNES_NETWORK".into()))
}
fn bounded(reader: impl Read, limit: usize) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    reader
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "METADATA_NETWORK".to_string())?;
    if bytes.len() > limit {
        return Err("METADATA_IMAGE_SIZE".into());
    }
    Ok(bytes)
}
pub fn validate_cover_url(raw: &str) -> Result<(), String> {
    if raw.chars().any(char::is_control) {
        return Err("METADATA_COVER_URL".into());
    }
    let parsed = ureq::get(raw)
        .request_url()
        .map_err(|_| "METADATA_COVER_URL".to_string())?;
    let url = parsed.as_url();
    let host = url.host_str().unwrap_or_default();
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some_and(|p| p != 443)
        || url.query().is_some()
        || url.fragment().is_some()
        || !(host == "coverartarchive.org"
            || host == "archive.org"
            || host.ends_with(".archive.org"))
    {
        return Err("METADATA_COVER_URL".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn itunes_cn_empty_or_network_error_falls_back_to_us_without_losing_success() {
        for network_error in [false, true] {
            let mut calls = Vec::new();
            let value = country_fallback(|country| {
                calls.push(country.to_string());
                if country == "cn" && network_error { return Err("METADATA_ITUNES_NETWORK".into()); }
                Ok(serde_json::json!({"results": if country == "cn" { vec![] } else { vec![serde_json::json!({"trackName":"US match"})] }}))
            }).unwrap();
            assert_eq!(calls, vec!["cn", "us"]);
            assert_eq!(value["results"][0]["trackName"], "US match");
        }
        assert_eq!(
            country_fallback(|_| Err("METADATA_ITUNES_NETWORK".into())).unwrap_err(),
            "METADATA_ITUNES_NETWORK"
        );
        assert_eq!(
            country_fallback(|_| Ok(serde_json::json!({}))).unwrap_err(),
            "METADATA_ITUNES_RESPONSE"
        );
    }
    #[test]
    fn streaming_limits_do_not_trust_headers_and_errors_are_fixed_codes() {
        assert_eq!(bounded(&b"1234"[..], 4).unwrap(), b"1234");
        assert_eq!(
            bounded(&b"12345"[..], 4).unwrap_err(),
            "METADATA_IMAGE_SIZE"
        );
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("private path and upstream body"))
            }
        }
        assert_eq!(bounded(Broken, 4).unwrap_err(), "METADATA_NETWORK");
        assert!(valid_mbid("12345678-1234-1234-1234-123456789abc"));
        assert!(!valid_mbid("../release"));
    }
}
