use crate::helpers::to_string;

pub use self::simplified_item::{PlayableId, SimplifiedItem};
use rspotify::model::{AdditionalType, TrackId};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, ClientError};
use serde::Serialize;
use std::sync::Mutex as SyncMutex;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tauri::async_runtime::{JoinHandle, Mutex};
use tauri::Manager;
use thiserror::Error;

mod simplified_item;

pub struct EventLoopHandle(pub SyncMutex<Option<JoinHandle<()>>>);

pub const STORE_PATH_BUF: &str = "store.bin";
pub const STORE_TOKEN_KEY: &str = "access_token";

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub prev: Option<SimplifiedItem>,
    pub curr: Option<SimplifiedItem>,
    pub next: Option<SimplifiedItem>,
    pub shuffle: bool,
    pub progress_ms: u64,
    pub playing: bool,

    #[serde(skip_serializing)]
    pub last_playback_call: Instant,
    #[serde(skip_serializing)]
    pub spotify_client: Arc<Mutex<AuthCodeSpotify>>,
    #[serde(skip_serializing)]
    pub last_seek_update: Instant,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
}

#[derive(Error, Serialize, Debug)]
pub enum GetCurrentPlaybackError {
    #[serde(serialize_with = "to_string")]
    #[error(transparent)]
    SpotifyClientError(ClientError),
    #[error("Spotify token not set")]
    TokenNotSet,
}
impl From<ClientError> for GetCurrentPlaybackError {
    fn from(error: ClientError) -> Self {
        Self::SpotifyClientError(error)
    }
}

impl AppState {
    pub const PLAYBACK_CALL_BUFFER: Duration = Duration::from_secs(5);
    pub const SEEK_CALL_BUFFER: Duration = Duration::from_millis(1);

    pub fn emit_update(&self, app_handle: &tauri::AppHandle) {
        app_handle
            .emit_all("app_state_change", self.clone())
            .unwrap();
    }

    pub async fn current_user_saved_tracks_contains(&self, id: &TrackId<'static>) -> bool {
        let spotify_client = self.spotify_client.lock().await;
        let saved_tracks = (*spotify_client)
            .current_user_saved_tracks_contains(vec![id.clone()])
            .await;

        if let Ok(tracks) = saved_tracks {
            match tracks[..] {
                [liked] => liked,
                _ => false,
            }
        } else {
            false
        }
    }

    pub async fn get_current_playback(&mut self) -> Result<(), GetCurrentPlaybackError> {
        use AdditionalType::*;

        let context = {
            let spotify_client = self.spotify_client.clone();
            let spotify_client = spotify_client.lock().await;
            {
                let token = spotify_client.token.lock().await.unwrap();
                token.as_ref().ok_or(GetCurrentPlaybackError::TokenNotSet)?;
            }
            (*spotify_client)
                .current_playback(None, Some(vec![Track, Episode].iter()))
                .await?
        };

        if let Some(context) = context {
            self.playing = context.is_playing;
            self.device_id = context.device.id.clone();
            self.shuffle = context.shuffle_state;

            let mut item = SimplifiedItem::from(context);

            item.saved = match &item.id {
                Some(PlayableId::Track(id)) => self.current_user_saved_tracks_contains(id).await,
                _ => false,
            };

            self.progress_ms = item.progress_ms;
            self.curr = Some(item);
        }

        self.last_playback_call = Instant::now();

        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            spotify_client: Arc::default(),
            prev: None,
            curr: None,
            next: None,
            shuffle: false,
            progress_ms: 0,
            playing: false,
            last_playback_call: Instant::now(),
            last_seek_update: Instant::now(),
            device_id: None,
        }
    }
}

pub struct AppStore(pub Arc<Mutex<AppState>>);
