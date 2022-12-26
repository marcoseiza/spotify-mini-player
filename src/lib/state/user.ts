import { invoke } from "@tauri-apps/api/tauri";
import { writable, type Readable } from "svelte/store";

type User = Readable<boolean> & { login: () => Promise<void> };
export const user: User = (() => {
  const { subscribe, set } = writable<boolean>();

  return {
    subscribe,
    login: async () => {
      const user = await invoke<any>("login_spotify");
      set(true);
    },
  };
})();
