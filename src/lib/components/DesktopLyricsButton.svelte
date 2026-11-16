<script lang="ts">
  import { Captions } from 'lucide-svelte';
  import IconButton from './IconButton.svelte';
  import { ui } from '../stores/ui.svelte';
  import { toggleDesktopLyrics } from '../stores/lyrics-overlay';

  let label = $derived(
    ui.lyricsOverlayLocked ? '解锁桌面歌词' : ui.lyricsOverlayOpen ? '关闭桌面歌词' : '桌面歌词',
  );
</script>

<IconButton
  class="desktop-lyrics-toggle"
  {label}
  active={ui.lyricsOverlayOpen || ui.lyricsOverlayLocked}
  aria-pressed={ui.lyricsOverlayOpen}
  onclick={() => toggleDesktopLyrics()}><Captions size={17} /></IconButton
>
