import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { writable, type Readable, type Subscriber } from "svelte/store";

type AsyncReadable<T> = Readable<T> & { init: () => Promise<void> };

interface Playback {
  item: {
    name: string;
    duration_ms: number;
    album: {
      images: { height: number; url: string; width: number }[];
    };
    artists: {
      external_urls: { spotify: string };
      name: string;
    }[];
  };
  progress_ms: number;
}

export const playback: AsyncReadable<Playback> = (() => {
  const { subscribe, set } = writable<Playback>();
  let unsubscribe: UnlistenFn = () => {};

  return {
    subscribe: (
      run: Subscriber<Playback>,
      invalidate?: (value?: Playback) => void
    ) => {
      subscribe(run, invalidate);
      return unsubscribe;
    },
    init: async () => {
      unsubscribe = await listen<Playback>("playback", (e) => {
        set(e.payload);
        console.log(e.payload);
      });
    },
  };
})();
