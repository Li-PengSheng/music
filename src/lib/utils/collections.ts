/**
 * Built-in collections are addressed by route kind, never by display name, so a user playlist that happens to be
 * called “本地” keeps its own ID, songs, rename and delete.
 */
export const LOCAL_COLLECTION = { view: 'local', name: '本地' } as const;

/** Every track in the library: no stored membership, so imports, rescans and removed files update it automatically. */
export function localQuery(sort: string, search: string) {
  return { sort, search };
}

const norm = (name: string) => name.trim().toLowerCase();

/** True for a user playlist whose name collides with a built-in collection; the UI labels it instead of changing it. */
export function shadowsBuiltIn(name: string) {
  return norm(name) === norm(LOCAL_COLLECTION.name);
}
