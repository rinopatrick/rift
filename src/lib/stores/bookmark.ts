import { invoke } from "@tauri-apps/api/core";

export interface Bookmark {
  id: string;
  connection_id: string;
  name: string;
  query: string;
  created_at: string;
}

export function createBookmarkStore() {
  let bookmarks = $state<Bookmark[]>([]);
  let loading = $state(false);

  async function load(connectionId: string) {
    loading = true;
    try {
      bookmarks = await invoke<Bookmark[]>("get_bookmarks", { connectionId });
    } finally {
      loading = false;
    }
  }

  async function save(connectionId: string, name: string, query: string) {
    await invoke("save_bookmark", { connectionId, name, query });
    await load(connectionId);
  }

  async function remove(id: string, connectionId: string) {
    await invoke("delete_bookmark", { id });
    await load(connectionId);
  }

  function clear() {
    bookmarks = [];
  }

  return {
    get bookmarks() { return bookmarks; },
    get loading() { return loading; },
    load,
    save,
    remove,
    clear,
  };
}

export const bookmarkStore = createBookmarkStore();
