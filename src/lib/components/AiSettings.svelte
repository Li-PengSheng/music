<script lang="ts">
  import { onMount } from 'svelte';
  import { aiConfig, desktopAvailable, type AiConfig } from '../services/music';
  let config = $state<AiConfig | null>(null);
  let baseUrl = $state('https://api.openai.com/v1');
  let model = $state('gpt-4o-mini');
  let apiKey = $state('');
  let busy = $state(false);
  let message = $state('');
  let disposed = false;
  onMount(() => {
    let alive = true;
    if (desktopAvailable)
      void aiConfig()
        .then((value) => {
          if (!alive) return;
          config = value;
          baseUrl = value.base_url;
          model = value.model;
        })
        .catch(() => {
          if (alive) message = '无法读取 AI 设置';
        });
    return () => {
      alive = false;
      disposed = true;
      apiKey = '';
    };
  });
  async function save() {
    if (busy || !config) return;
    busy = true;
    try {
      const result = await aiConfig(null, baseUrl, apiKey || null, model);
      if (disposed) return;
      config = result;
      baseUrl = config.base_url;
      model = config.model;
      apiKey = '';
      message = 'AI 设置已保存';
    } catch (e) {
      if (!disposed) message = String(e);
    } finally {
      busy = false;
    }
  }
  async function toggle(enabled: boolean) {
    if (busy || !config) return;
    busy = true;
    try {
      const result = await aiConfig(enabled);
      if (disposed) return;
      config = result;
      message = enabled ? 'AI 助手已开启' : 'AI 助手已关闭';
    } catch (e) {
      if (!disposed) message = String(e);
    } finally {
      busy = false;
    }
  }
  async function clear() {
    if (busy) return;
    busy = true;
    try {
      const result = await aiConfig(false, null, '');
      if (disposed) return;
      config = result;
      apiKey = '';
      message = '已清除密钥并关闭 AI 助手';
    } catch (e) {
      if (!disposed) message = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<section class="settings-card" aria-labelledby="set-ai">
  <h3 id="set-ai">AI 助手</h3>
  <div class="settings-row">
    <div>
      <strong>启用 AI 助手</strong>
      <p>默认关闭。自然语言对话会发送到下方服务地址；工具只使用必要的曲库元数据。</p>
    </div>
    <label class="switch"
      ><input
        type="checkbox"
        role="switch"
        checked={config?.enabled ?? false}
        disabled={busy || !config}
        onchange={async (event) => {
          const input = event.currentTarget;
          await toggle(input.checked);
          if (!disposed) input.checked = config?.enabled ?? false;
        }}
      />
      <span aria-hidden="true"></span><span class="sr-only">启用 AI 助手</span></label
    >
  </div>
  <form
    class="ai-form"
    onsubmit={(event) => {
      event.preventDefault();
      void save();
    }}
  >
    <label
      >Base URL<input
        class="input"
        type="url"
        bind:value={baseUrl}
        maxlength="2048"
        required
        disabled={busy || !config}
        placeholder="https://api.openai.com/v1"
      /></label
    >
    <label
      >模型名称<input
        class="input"
        bind:value={model}
        maxlength="128"
        required
        disabled={busy || !config}
        placeholder="gpt-4o-mini"
      /></label
    >
    <label
      >API Key<input
        class="input"
        type="password"
        bind:value={apiKey}
        maxlength="512"
        autocomplete="new-password"
        disabled={busy || !config}
        placeholder={config?.has_key ? '已保存，留空保留现有密钥' : '填写服务 API Key'}
      /></label
    >
    <div class="settings-actions">
      <button type="submit" class="btn btn-secondary" disabled={busy || !config}>保存设置</button>
      <button
        type="button"
        class="btn btn-ghost"
        disabled={busy || !config?.has_key}
        onclick={() => void clear()}>清除密钥</button
      >
    </div>
  </form>
  <p class="settings-note">
    使用 Chat Completions 兼容地址，可配置 OpenAI、Gemini、DeepSeek、OpenRouter 或本机
    Ollama；模型需支持工具调用。
  </p>
  <p class="settings-note">
    密钥保存在本机设置文件中。无需密钥的本地服务可填写占位值。对话仅保留在本次控制台会话。
  </p>
  <p class="settings-note" role="status" aria-live="polite">{message}</p>
</section>

<style>
  .ai-form {
    display: grid;
    gap: var(--space-4);
    margin-top: var(--space-4);
  }
  .ai-form label {
    display: grid;
    gap: var(--space-2);
    color: var(--text-2);
    font-size: var(--fs-meta);
  }
  .ai-form input {
    width: 100%;
    min-width: 0;
  }
  .ai-form .settings-actions {
    flex-wrap: wrap;
  }
</style>
