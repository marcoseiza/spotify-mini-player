use rspotify::model::{
    CurrentPlaybackContext, EpisodeId, FullEpisode, FullTrack, Image, PlayableItem,
    SimplifiedAlbum, SimplifiedArtist, TrackId,
};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum PlayableId {
    Track(TrackId<'static>),
    Episode(EpisodeId<'static>),
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedItem {
    pub context_uri: Option<String>,
    pub id: Option<PlayableId>,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub album: Option<SimplifiedAlbum>,
    pub artists: Vec<SimplifiedArtist>,
    pub saved: bool,
    pub duration_ms: u64,
    pub progress_ms: u64,
}

impl SimplifiedItem {
    const PREFERRED_IMAGE_WIDTH_PX: u32 = 200;
    fn get_id(playback: &CurrentPlaybackContext) -> Option<PlayableId> {
        match &playback.item {
            Some(PlayableItem::Track(t)) => t.id.as_ref().map(|t| PlayableId::Track(t.clone())),
            Some(PlayableItem::Episode(e)) => Some(PlayableId::Episode(e.id.clone())),
            _ => None,
        }
    }

    fn get_artists(playback: &CurrentPlaybackContext) -> Vec<SimplifiedArtist> {
        match &playback.item {
            Some(PlayableItem::Track(t)) => t.artists.clone(),
            _ => Vec::new(),
        }
    }

    fn get_album(playback: &CurrentPlaybackContext) -> Option<SimplifiedAlbum> {
        match &playback.item {
            Some(PlayableItem::Track(t)) => Some(t.album.clone()),
            _ => None,
        }
    }

    fn get_preferred_image(images: &[Image]) -> Option<&Image> {
        images.iter().reduce(|accum, item| {
            let preferred = Self::PREFERRED_IMAGE_WIDTH_PX as i64;
            let dist_to_accum = (preferred - accum.width.unwrap_or_default() as i64).abs();
            let dist_to_item = (preferred - item.width.unwrap_or_default() as i64).abs();
            if dist_to_accum <= dist_to_item {
                accum
            } else {
                item
            }
        })
    }

    fn get_image_url(playback: &CurrentPlaybackContext) -> Option<String> {
        match &playback.item {
            Some(PlayableItem::Track(FullTrack { album, .. })) => {
                Self::get_preferred_image(&album.images).map(|image| image.url.clone())
            }
            Some(PlayableItem::Episode(FullEpisode { images, .. })) => {
                Self::get_preferred_image(images).map(|image| image.url.clone())
            }
            _ => None,
        }
    }

    fn get_duration_ms(playback: &CurrentPlaybackContext) -> u64 {
        match &playback.item {
            Some(PlayableItem::Track(t)) => t.duration.as_millis() as u64,
            Some(PlayableItem::Episode(t)) => t.duration.as_millis() as u64,
            _ => Default::default(),
        }
    }

    fn get_name(playback: &CurrentPlaybackContext) -> Option<String> {
        match &playback.item {
            Some(PlayableItem::Track(t)) => Some(t.name.clone()),
            Some(PlayableItem::Episode(t)) => Some(t.name.clone()),
            _ => None,
        }
    }
}

impl From<CurrentPlaybackContext> for SimplifiedItem {
    fn from(playback: CurrentPlaybackContext) -> Self {
        Self {
            id: Self::get_id(&playback),
            name: Self::get_name(&playback),
            image_url: Self::get_image_url(&playback),
            artists: Self::get_artists(&playback),
            album: Self::get_album(&playback),
            duration_ms: Self::get_duration_ms(&playback),
            progress_ms: playback.progress.unwrap_or_default().as_millis() as u64,
            context_uri: playback.context.map(|c| c.uri),
            ..Self::default()
        }
    }
}
