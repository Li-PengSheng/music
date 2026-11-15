export type Shortcut = 'toggle' | 'search' | 'previous' | 'next' | 'lyrics' | 'open' | 'escape';
export const shortcutHelp = [
  ['Space', '播放 / 暂停'],
  ['Ctrl+F', '聚焦搜索'],
  ['Ctrl+←', '上一首'],
  ['Ctrl+→', '下一首'],
  ['Ctrl+L', '打开 / 关闭歌词'],
  ['Ctrl+O', '选择本地音乐文件'],
  ['Escape', '关闭弹窗或退出沉浸页面'],
];
export function shortcut(
  e: Pick<
    KeyboardEvent,
    | 'key'
    | 'ctrlKey'
    | 'altKey'
    | 'metaKey'
    | 'shiftKey'
    | 'repeat'
    | 'isComposing'
    | 'defaultPrevented'
  >,
  editable: boolean,
  interactive: boolean,
  modal: boolean,
): Shortcut | null {
  if (e.defaultPrevented || e.repeat || e.isComposing || e.altKey || e.metaKey || e.shiftKey)
    return null;
  if (e.key === 'Escape') return 'escape';
  if (modal) return null;
  if (e.ctrlKey) {
    if (e.key.toLowerCase() === 'f') return 'search';
    if (editable) return null;
    return (
      (
        { ArrowLeft: 'previous', ArrowRight: 'next', l: 'lyrics', o: 'open' } as Record<
          string,
          Shortcut
        >
      )[e.key.length === 1 ? e.key.toLowerCase() : e.key] ?? null
    );
  }
  return e.key === ' ' && !editable && !interactive ? 'toggle' : null;
}
