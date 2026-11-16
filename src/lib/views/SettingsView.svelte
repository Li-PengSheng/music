<script lang="ts">
  import { onMount } from 'svelte';
  import { FolderPlus, Database, X } from 'lucide-svelte';
  import Select from '../components/Select.svelte';
  import { shortcutHelp } from '../utils/shortcuts';
  import {
    desktopAvailable,
    mediaStatus,
    desktopPreferences,
    cancelScan,
    removeFolder,
    clearCoverCaches,
  } from '../services/music';
  import { player } from '../stores/player.svelte';
  import { library, refreshSummary } from '../stores/library.svelte';
  import { ui, notify, notifyError, confirmAction } from '../stores/ui.svelte';
  import { importFolder, importLegacyDatabase } from '../actions';
  import { setReduceMotion, setTheme } from '../stores/preferences';
  import { parseTheme } from '../utils/theme';

  let media = $state('尚未连接桌面');
  let closeToTray = $state(false);
  let desktopReady = $state(false);
  let saving = $state(false);
  let importing = $state(false);
  let removing = $state<string | null>(null);
  let confirming = false;

  async function remove(folder: string) {
    if (confirming || removing !== null || scanning || !desktopAvailable) return;
    confirming = true;
    try {
      if (
        !(await confirmAction({
          title: '移除音乐文件夹？',
          message: '仅从音乐库移除记录，不会删除磁盘上的任何音乐文件。',
          detail: folder,
          confirmLabel: '移除',
          danger: true,
        }))
      )
        return;
      removing = folder;
      await removeFolder(folder);
      await refreshSummary();
      clearCoverCaches();
      library.coverEpoch++;
      notify('已从音乐库移除文件夹，磁盘上的音乐文件未被删除');
    } catch (e) {
      notifyError(e);
    } finally {
      removing = null;
      confirming = false;
    }
  }

  onMount(() => {
    if (!desktopAvailable) return;
    void mediaStatus()
      .then((s) => (media = s))
      .catch(notifyError);
    void desktopPreferences()
      .then((p) => {
        closeToTray = p.close_to_tray;
        desktopReady = true;
      })
      .catch(notifyError);
  });

  async function changeClose(value: string) {
    saving = true;
    try {
      const enabled = value === 'tray';
      const p = await desktopPreferences(enabled);
      closeToTray = p.close_to_tray;
      if (enabled) notify('关闭窗口后音乐继续播放；右键托盘图标选择“退出”可结束程序。');
    } catch (e) {
      notifyError(e);
    } finally {
      saving = false;
    }
  }
  async function legacy() {
    importing = true;
    try {
      await importLegacyDatabase();
    } finally {
      importing = false;
    }
  }
  let scanning = $derived(!!library.progress?.running);
</script>

<header class="page-header">
  <div>
    <h2 class="page-title">设置</h2>
    <p class="page-meta">Music Mine v0.1.0 · 本地离线运行</p>
  </div>
</header>

<div class="settings">
  <section class="settings-card" aria-labelledby="set-library">
    <h3 id="set-library">音乐库</h3>
    <p class="settings-note">Music Mine 只读取音乐文件和标签，从不修改、移动或删除它们。</p>
    {#if library.summary.folders.length}
      <ul class="folder-list">
        {#each library.summary.folders as folder (folder)}<li class="mono" title={folder}>
            <span class="folder-path">{folder}</span>
            <button
              type="button"
              class="btn btn-ghost btn-sm folder-remove"
              aria-label={'移除音乐文件夹：' + folder}
              disabled={scanning || removing !== null || !desktopAvailable}
              onclick={() => remove(folder)}>移除</button
            >
          </li>{/each}
      </ul>
    {:else}<p class="settings-note">尚未添加音乐文件夹。</p>{/if}
    <div class="settings-actions">
      <button
        type="button"
        class="btn btn-primary"
        onclick={importFolder}
        disabled={scanning || !desktopAvailable}
        ><FolderPlus size={16} />添加或重新扫描文件夹</button
      >
    </div>
    {#if library.progress && (library.progress.running || library.progress.visited || library.progress.errors.length)}
      {@const p = library.progress}
      <div class="scan-status" aria-live="polite">
        <div class="scan-line">
          <strong
            >{p.running
              ? '正在扫描'
              : p.cancelled
                ? '扫描已取消，已导入的歌曲会保留'
                : '扫描完成'}</strong
          ><span>已检查 {p.visited} · 已导入或更新 {p.imported}</span>
          {#if p.running}<button
              type="button"
              class="btn btn-ghost btn-sm"
              onclick={() => cancelScan().catch(notifyError)}><X size={14} />取消</button
            >{/if}
        </div>
        {#if p.current}<small class="mono">{p.current}</small>{/if}
        {#if p.errors.length}<details>
            <summary>{p.errors.length} 条错误</summary>{#each p.errors as e, i (i)}<p class="mono">
                {e}
              </p>{/each}
          </details>{/if}
      </div>
    {/if}
  </section>

  <section class="settings-card" aria-labelledby="set-playback">
    <h3 id="set-playback">播放</h3>
    <dl class="settings-list">
      <div>
        <dt>音频输出</dt>
        <dd>{player.snapshot?.output_device_id ?? '尚未打开音频输出设备'}</dd>
      </div>
      <div>
        <dt>Windows 媒体控制</dt>
        <dd>{media}</dd>
      </div>
    </dl>
    <p class="settings-note">播放队列、进度、音量和播放模式会自动保存，重新启动后保持暂停。</p>
  </section>

  <section class="settings-card" aria-labelledby="set-appearance">
    <h3 id="set-appearance">外观</h3>
    <div class="settings-row">
      <div>
        <strong>主题</strong>
        <p>同时应用于窗口标题栏。“跟随系统”会随 Windows 的深色/浅色设置自动切换。</p>
      </div>
      <Select
        label="主题"
        value={ui.theme}
        onchange={(v) => setTheme(parseTheme(v))}
        options={[
          { value: 'dark', label: '深色' },
          { value: 'light', label: '浅色' },
          { value: 'system', label: '跟随系统' },
        ]}
      />
    </div>
    <div class="settings-row">
      <div>
        <strong>减少动态效果</strong>
        <p>关闭过渡动画与正在播放页面的氛围光晕。系统已开启“减少动画”时自动生效。</p>
      </div>
      <label class="switch"
        ><input
          type="checkbox"
          role="switch"
          checked={ui.reduceMotion}
          onchange={(e) => setReduceMotion(e.currentTarget.checked)}
        /><span aria-hidden="true"></span><span class="sr-only">减少动态效果</span></label
      >
    </div>
  </section>

  <section class="settings-card" aria-labelledby="set-window">
    <h3 id="set-window">窗口</h3>
    <div class="settings-row">
      <div>
        <strong>关闭主窗口时</strong>
        <p>选择托盘后，左键托盘图标恢复窗口，右键可控制播放或退出。</p>
      </div>
      <Select
        label="关闭主窗口时"
        value={closeToTray ? 'tray' : 'exit'}
        disabled={!desktopReady || saving}
        onchange={changeClose}
        options={[
          { value: 'exit', label: '退出程序' },
          { value: 'tray', label: '最小化到系统托盘' },
        ]}
      />
    </div>
  </section>

  <section class="settings-card" aria-labelledby="set-keys">
    <h3 id="set-keys">键盘快捷键</h3>
    <dl class="shortcut-list">
      {#each shortcutHelp as [key, label] (key)}<div>
          <dt><kbd>{key}</kbd></dt>
          <dd>{label}</dd>
        </div>{/each}
    </dl>
    <p class="settings-note">
      仅在窗口内生效，不占用其他应用的全局快捷键。输入文字时保留编辑操作。
    </p>
  </section>

  <section class="settings-card" aria-labelledby="set-legacy">
    <h3 id="set-legacy">从旧版迁移</h3>
    <p class="settings-note">
      选择旧版数据库，制作一致性副本后导入。仅支持空音乐库；旧数据库与音乐文件保持只读。
    </p>
    <div class="settings-actions">
      <button
        type="button"
        class="btn btn-secondary"
        onclick={legacy}
        disabled={importing || scanning || !desktopAvailable}
        ><Database size={16} />{importing ? '正在备份与校验…' : '导入旧版数据库'}</button
      >
    </div>
  </section>
</div>

<style>
  :global {
    /* ============================================================ settings */
    .settings {
      display: flex;
      flex-direction: column;
      gap: var(--space-4);
      max-width: 760px;
    }

    .settings-card {
      padding: var(--space-5);
      border-radius: var(--radius-lg);
      background: var(--surface);
    }

    .settings-card h3 {
      margin-bottom: var(--space-3);
      font-size: 15px;
      font-weight: var(--fw-semibold);
    }

    .settings-note {
      color: var(--text-2);
      font-size: var(--fs-meta);
      line-height: 1.6;
    }

    .settings-actions {
      display: flex;
      gap: 8px;
      margin-top: var(--space-4);
    }

    .folder-list {
      display: flex;
      flex-direction: column;
      gap: 4px;
      margin-top: var(--space-3);
    }

    .folder-list li {
      position: relative;
      padding: 8px 12px;
      border-radius: var(--radius-sm);
      background: var(--bg);
      color: var(--text-2);
    }

    .folder-path {
      display: block;
      padding-right: 64px;
    }
    .folder-remove {
      position: absolute;
      right: 6px;
      top: 50%;
      transform: translateY(-50%);
      opacity: 0;
    }
    .folder-list li:hover .folder-remove,
    .folder-list li:focus-within .folder-remove {
      opacity: 1;
    }

    .scan-status {
      display: flex;
      flex-direction: column;
      gap: 6px;
      margin-top: var(--space-4);
      padding: var(--space-3);
      border-radius: var(--radius);
      background: var(--bg);
      font-size: var(--fs-meta);
    }

    .scan-line {
      display: flex;
      align-items: center;
      gap: 12px;
      flex-wrap: wrap;
    }

    .scan-line span {
      color: var(--text-2);
    }

    .scan-status small {
      color: var(--text-3);
    }

    .scan-status details {
      color: var(--danger);
    }

    .settings-list {
      display: grid;
      gap: 10px;
    }

    .settings-list > div,
    .settings-row {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--space-4);
    }

    .settings-list dt {
      color: var(--text-2);
      font-size: var(--fs-meta);
    }

    .settings-list dd {
      color: var(--text);
      font-size: var(--fs-meta);
      text-align: right;
      overflow-wrap: anywhere;
    }

    .settings-row {
      padding: var(--space-3) 0;
      border-top: 1px solid var(--line);
    }

    .settings-row:first-of-type {
      border-top: 0;
      padding-top: 0;
    }

    .settings-row strong {
      font-size: var(--fs-body);
      font-weight: var(--fw-semibold);
    }

    .settings-row p {
      margin-top: 2px;
      color: var(--text-3);
      font-size: var(--fs-small);
      line-height: 1.5;
    }

    .shortcut-list {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
      gap: 6px 24px;
      margin-bottom: var(--space-3);
    }

    .shortcut-list > div {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 12px;
      padding: 6px 0;
      border-bottom: 1px solid var(--line);
    }

    .shortcut-list dd {
      color: var(--text-2);
      font-size: var(--fs-meta);
    }
  }
</style>
