<script lang="ts">
  import ContextMenu from '../../src/lib/components/ContextMenu.svelte';
  import Modal from '../../src/lib/components/Modal.svelte';
  import Slider from '../../src/lib/components/Slider.svelte';
  import { tooltip } from '../../src/lib/components/tooltip';
  import { openMenu, ui } from '../../src/lib/stores/ui.svelte';
  let selected = $state('');
  let modal = $state(false);
  let busy = $state(false);
  let volume = $state(50);
  function menu(at: MouseEvent | HTMLElement) {
    openMenu(at, [
      { label: '播放', action: () => (selected = 'play') },
      { label: '不可用', disabled: true, action: () => (selected = 'disabled') },
      { label: '下一首播放', action: () => (selected = 'next') },
      { kind: 'separator' },
      { label: '加入播放队列', action: () => (selected = 'queue') },
      {
        label: '失败操作',
        action: () => {
          throw new Error('test action failed');
        },
      },
    ]);
  }
</script>

<main style="padding: 24px">
  <button oncontextmenu={menu} onclick={(e) => menu(e.currentTarget)}>操作</button>
  <button onclick={() => (modal = true)}>打开弹窗</button>
  <button>外部按钮</button>
  <Slider value={volume} label="测试音量" oninput={(v) => (volume = v)} />
  <output aria-label="选择结果">{selected}</output>
  <output aria-label="音量结果">{volume}</output>
  <output aria-label="错误">{ui.toast?.message ?? ''}</output>
</main>
<button style="position:fixed;right:8px;bottom:8px" oncontextmenu={menu} use:tooltip={'边缘提示'}
  >边缘操作</button
>
{#if modal}
  <Modal
    onclose={() => (modal = false)}
    dismissible={!busy}
    initialFocus="#fixture-input"
    aria-label="测试弹窗"
  >
    <div style="padding:24px;min-height:260px">
      <label for="fixture-input">名称</label><input
        id="fixture-input"
        class="input"
        value="原始内容"
      />
      <button onclick={() => (busy = !busy)}>切换忙碌</button>
      <button onclick={() => (modal = false)}>关闭</button>
    </div>
  </Modal>
{/if}
<ContextMenu />
