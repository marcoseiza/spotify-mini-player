import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { writable, type Readable, get, type Subscriber } from "svelte/store";
import type { SimplifiedItem } from "./simplified-item";
import { invoke } from "@tauri-apps/api";

const REPEAT_STATE = {
  OFF: "off",
  TRACK: "track",
  CONTEXT: "context",
} as const;
export type RepeatState = typeof REPEAT_STATE[keyof typeof REPEAT_STATE];

interface AppState {
  curr: SimplifiedItem | undefined;
  playing: boolean;
  repeatState: RepeatState;
  progressMs: number;
  shuffle: boolean;
}

const defaultAppStore: AppState = {
  curr: undefined,
  playing: false,
  repeatState: "off",
  progressMs: 0,
  shuffle: false,
};

type Invalidator<T> = (value?: T) => void;

export type AppStore = Readable<AppState> & {
  init: () => Promise<void>;
};

export const appStore = (() => {
  const store = writable<AppState>(defaultAppStore);
  const { subscribe, set: setStore } = store;
  const listenerUnsubscribeList = new Map<String, UnlistenFn>();

  const set = (v: Partial<AppState>) => {
    const prevState = get(store);
    const newState = Object.assign(prevState, v) as AppState;
    setStore(newState);
  };

  const handlePlaybackPayload = (appState: AppState) => {
    set(appState);
  };

  const initOnPlayback = (): Promise<UnlistenFn> => {
    return listen<AppState>("app_state_change", (e) => {
      handlePlaybackPayload(e.payload);
    });
  };

  return {
    subscribe: (
      run: Subscriber<AppState>,
      invalidate?: Invalidator<AppState> | undefined
    ) => {
      subscribe(run, invalidate);
      return () => {
        for (const unsubscribe of listenerUnsubscribeList.values()) {
          unsubscribe();
        }
      };
    },
    init: async () => {
      listenerUnsubscribeList.set("app_state_change", await initOnPlayback());
    },
    playPause: async () => {
      await invoke("play_pause");
    },
    prevTrack: async () => {
      await invoke("prev_track");
    },
    nextTrack: async () => {
      await invoke("next_track");
    },
    toggleSaved: async () => {
      await invoke("toggle_saved");
    },
    toggleShuffle: async () => {
      await invoke("toggle_shuffle");
    },
    cycleRepeatState: async () => {
      await invoke("cycle_repeat_state");
    },
  };
})();
