export interface SimplifiedAlbum {
  album_group?: string;
  album_type: string | undefined;
  available_markets?: string[];
  external_urls: Record<string, string>;
  href: string | undefined;
  id: string | undefined;
  name: string;
  release_date?: string;
  release_date_precision?: string;
}
