<script lang="ts">
  // Native range input (keyboard + screen reader support) with a shared visual style.
  let {
    value,
    min = 0,
    max = 100,
    step = 1,
    label,
    valuetext,
    disabled = false,
    class: className = '',
    oninput,
    onchange,
  }: {
    value: number;
    min?: number;
    max?: number;
    step?: number;
    label: string;
    valuetext?: string;
    disabled?: boolean;
    class?: string;
    oninput?: (value: number) => void;
    onchange?: (value: number) => void;
  } = $props();
  let fill = $derived(
    max > min ? Math.min(100, Math.max(0, ((value - min) / (max - min)) * 100)) : 0,
  );
</script>

<input
  class={['slider', className]}
  type="range"
  {min}
  {max}
  {step}
  {value}
  {disabled}
  aria-label={label}
  aria-valuetext={valuetext}
  style:--fill={fill + '%'}
  oninput={(e) => oninput?.(Number(e.currentTarget.value))}
  onchange={(e) => onchange?.(Number(e.currentTarget.value))}
/>
