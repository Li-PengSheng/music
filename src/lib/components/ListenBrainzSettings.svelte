<script lang="ts">
  import { onMount } from 'svelte';
  import { desktopAvailable, listenbrainzConfig, type ListenBrainzConfig } from '../services/music';

  let config = $state<ListenBrainzConfig | null>(null);
  let username = $state('');
  let token = $state('');
  let busy = $state(false);
  let message = $state('');
  let disposed = false;

  onMount(() => {
    if (!desktopAvailable) return;
    let alive = true;
    void listenbrainzConfig()
      .then((value) => {
        if (!alive) return;
        config = value;
        username = value.username ?? '';
      })
      .catch(() => {
        if (alive) message = '无法读取 ListenBrainz 设置';
      });
    return () => {
      alive = false;
      disposed = true;
      token = '';
    };
  });

  async function save() {
    if (busy) return;
    busy = true;
    message = '正在验证连接…';
    try {
      const result = await listenbrainzConfig(null, username, token || null);
      if (disposed) return;
      config = result;
      username = config.username ?? '';
      token = '';
      message = '连接成功，设置已保存';
    } catch (error) {
      // Rust exposes only sanitized errors. Never echo the entered credential.
      if (!disposed) message = String(error);
    } finally {
      busy = false;
    }
  }
  async function toggle(enabled: boolean) {
    if (!config || busy) return;
    busy = true;
    try {
      const result = await listenbrainzConfig(enabled);
      if (disposed) return;
      config = result;
      message = config.enabled
        ? '已启用听歌记录与本地推荐'
        : '已关闭：停止采集与上传，待传记录保留';
    } catch (error) {
      if (!disposed) message = String(error);
    } finally {
      busy = false;
    }
  }
  async function clearToken() {
    if (busy) return;
    busy = true;
    try {
      const result = await listenbrainzConfig(false, '', '');
      if (disposed) return;
      config = result;
      username = '';
      token = '';
      message = '已关闭并清除凭证，待传记录保留在本地';
    } catch (error) {
      if (!disposed) message = String(error);
    } finally {
      busy = false;
    }
  }
</script>

<section class="settings-card" aria-labelledby="set-listenbrainz">
  <h3 id="set-listenbrainz">听歌记录 (ListenBrainz / Last.fm)</h3>
  <div class="settings-row">
    <div>
      <strong>启用 Scrobble</strong>
      <p>默认关闭。开启后发送曲名、艺术家、专辑与收听时间到 ListenBrainz，断网时稍后重试。</p>
    </div>
    <label class="switch">
      <input
        type="checkbox"
        role="switch"
        checked={config?.enabled ?? false}
        disabled={!config || busy}
        onchange={async (event) => {
          const input = event.currentTarget;
          await toggle(input.checked);
          if (!disposed) input.checked = config?.enabled ?? false;
        }}
      />
      <span aria-hidden="true"></span><span class="sr-only">启用 Scrobble</span>
    </label>
  </div>
  <form
    class="scrobble-form"
    onsubmit={(event) => {
      event.preventDefault();
      void save();
    }}
  >
    <label
      >ListenBrainz 用户名<input
        autocomplete="off"
        maxlength="128"
        bind:value={username}
        disabled={busy || !config}
      /></label
    >
    <label
      >User Token<input
        type="password"
        autocomplete="new-password"
        maxlength="256"
        bind:value={token}
        placeholder={config?.token_configured
          ? '已保存，留空保留现有 Token'
          : '粘贴 ListenBrainz User Token'}
        disabled={busy || !config}
      /></label
    >
    <div class="settings-actions">
      <button
        type="submit"
        class="btn btn-secondary"
        disabled={busy || !config || (!token && !config.token_configured)}>保存并测试连接</button
      >
      <button
        type="button"
        class="btn btn-ghost"
        disabled={busy || !config?.token_configured}
        onclick={() => void clearToken()}>清除凭证</button
      >
    </div>
  </form>
  <p class="settings-note">
    Token 仅保存在本机设置文件中。待上传 {config?.pending_count ?? 0} 条。关闭后，已发出的请求可能仍会完成。
  </p>
  <p class="settings-note">
    当前接入 ListenBrainz；如需同步 Last.fm，可在 ListenBrainz 网站关联账户。
  </p>
  <p class="settings-note" role="status" aria-live="polite">{message}</p>
</section>

<style>
  .scrobble-form {
    display: grid;
    gap: var(--space-4);
    margin-top: var(--space-4);
  }
  .scrobble-form label {
    display: grid;
    gap: var(--space-2);
    color: var(--text-2);
    font-size: var(--fs-meta);
  }
  .scrobble-form input {
    width: 100%;
    min-width: 0;
    padding: 8px 10px;
    border: 1px solid var(--line-strong);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    font: inherit;
  }
  .scrobble-form .settings-actions {
    flex-wrap: wrap;
  }
</style>
