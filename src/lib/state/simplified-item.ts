import type { Artist } from "./artist";
import type { SimplifiedAlbum } from "./simplified-album";

export interface SimplifiedItem {
  contextUri: string;
  id: string;
  name: string | undefined;
  imageUrl: string | undefined;
  album: SimplifiedAlbum | undefined;
  title: string;
  artists: Artist[];
  saved: boolean;
  durationMs: number;
  progressMs: number;
}
