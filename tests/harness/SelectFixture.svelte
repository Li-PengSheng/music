<script lang="ts">
  import Select from '../../src/lib/components/Select.svelte';
  import Modal from '../../src/lib/components/Modal.svelte';
  let value = $state('');
  let changes = $state<string[]>([]);
  let dialog = $state(false);
  const options = [
    { value: '', label: '选择模式' },
    { value: 'sequence', label: '顺序播放' },
    { value: 'shuffle', label: '随机播放' },
    { value: 'repeat', label: '单曲循环' },
  ];
</script>

<main style="padding: 24px">
  <Select label="播放模式" bind:value {options} onchange={(v) => changes.push(v)} />
  <button onclick={() => (value = 'shuffle')}>外部更新</button>
  <Select label="禁用选择" disabled {options} />
  <Select label="空列表" options={[]} />
  <output aria-label="绑定值">{value || '(empty)'}</output>
  <output aria-label="回调">{JSON.stringify(changes)}</output>
  <button onclick={() => (dialog = true)}>打开详情</button>
</main>

<div style="position: fixed; bottom: 8px; right: 8px">
  <Select
    label="长列表"
    options={Array.from({ length: 60 }, (_, i) => ({ value: String(i), label: `项目 ${i}` }))}
  />
</div>

{#if dialog}
  <Modal onclose={() => (dialog = false)} aria-label="选择器弹窗">
    <div style="padding: 24px; min-height: 260px">
      <Select label="弹窗模式" bind:value {options} />
      <button onclick={() => (dialog = false)}>关闭详情</button>
    </div>
  </Modal>
{/if}
