use rspotify::{prelude::OAuthClient, AuthCodeSpotify};
use rspotify::{ClientError, ClientResult};
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_store::{with_store, StoreCollection};
use thiserror::Error;

use crate::helpers::to_string;
use crate::redirect_uri::redirect_uri_web_server;
use crate::state::*;

#[derive(Error, Serialize, Debug)]
pub enum HandlerError {
    #[serde(serialize_with = "to_string")]
    #[error(transparent)]
    SpotifyClientError(ClientError),
    #[error(transparent)]
    GetCurrentPlaybackPlayback(GetCurrentPlaybackError),
    #[error("Handler error: {0}")]
    OtherError(String),
}

impl From<ClientError> for HandlerError {
    fn from(error: ClientError) -> Self {
        Self::SpotifyClientError(error)
    }
}
impl From<GetCurrentPlaybackError> for HandlerError {
    fn from(error: GetCurrentPlaybackError) -> Self {
        Self::GetCurrentPlaybackPlayback(error)
    }
}
impl From<String> for HandlerError {
    fn from(s: String) -> Self {
        Self::OtherError(s)
    }
}
impl From<&str> for HandlerError {
    fn from(s: &str) -> Self {
        Self::OtherError(s.into())
    }
}

pub async fn get_token_auto(spotify_oauth: &mut AuthCodeSpotify, port: u16) -> ClientResult<()> {
    match redirect_uri_web_server(spotify_oauth, port) {
        Ok(url) => {
            let code = spotify_oauth
                .parse_response_code(&url)
                .ok_or_else(|| ClientError::Cli("unable to parse the response code".to_string()))?;
            spotify_oauth.request_token(&code).await
        }
        Err(()) => Ok(()),
    }
}

#[tauri::command]
pub async fn login_spotify(
    app_handle: tauri::AppHandle,
    app_store: tauri::State<'_, AppStore>,
) -> Result<(), String> {
    let app_state = app_store.0.lock().await;
    let mut spotify = app_state.spotify_client.lock().await;

    if spotify.token.clone().lock().await.unwrap().is_none() {
        get_token_auto(&mut spotify, 8585).await.unwrap();

        let token_arc = spotify.token.clone();
        let token = token_arc.lock().await.unwrap();
        let serialized_token = serde_json::to_value(token.as_ref().unwrap()).unwrap();

        let collection = app_handle.state::<StoreCollection>();
        with_store(
            &app_handle,
            collection,
            STORE_PATH_BUF.parse().unwrap(),
            |store| {
                Ok(store
                    .cache
                    .insert(STORE_TOKEN_KEY.to_string(), serialized_token))
            },
        )
        .unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn play_pause(
    app_handle: tauri::AppHandle,
    app_store: tauri::State<'_, AppStore>,
) -> Result<(), HandlerError> {
    let mut app_state = app_store.0.lock().await;

    {
        let prev_app_state = app_state.clone();
        let spotify_client = app_state.spotify_client.clone();
        let spotify_client = spotify_client.lock().await;

        let result = if app_state.playing {
            app_state.playing = false;
            app_state.emit_update(&app_handle);
            (*spotify_client).pause_playback(None).await
        } else {
            app_state.playing = true;
            app_state.emit_update(&app_handle);
            let progress = Some(app_state.progress_ms as u32);
            let device_id = app_state.device_id.as_deref();
            (*spotify_client).resume_playback(device_id, progress).await
        };

        if result.is_err() {
            *app_state = prev_app_state;
            app_state.emit_update(&app_handle);
            result?;
        }
    }

    app_state.get_current_playback().await?;

    Ok(())
}

#[tauri::command]
pub async fn next_track(app_store: tauri::State<'_, AppStore>) -> Result<(), HandlerError> {
    let mut app_state = app_store.0.lock().await;

    {
        let spotify_client = app_state.spotify_client.lock().await;
        (*spotify_client).next_track(None).await?;
    }

    app_state.get_current_playback().await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_track(app_store: tauri::State<'_, AppStore>) -> Result<(), HandlerError> {
    let mut app_state = app_store.0.lock().await;

    {
        let spotify_client = app_state.spotify_client.lock().await;
        (*spotify_client).previous_track(None).await?;
    }

    app_state.get_current_playback().await?;

    Ok(())
}

#[tauri::command]
pub async fn toggle_saved(
    app_handle: tauri::AppHandle,
    app_store: tauri::State<'_, AppStore>,
) -> Result<(), HandlerError> {
    let mut app_state = app_store.0.lock().await;
    let prev_app_state = app_state.clone();

    let spotify_client = app_state.spotify_client.clone();
    let spotify_client = spotify_client.lock().await;

    let current = match &mut app_state.curr {
        Some(current) => current,
        None => return Err("No current playback".into()),
    };

    let id = match current.id.clone() {
        Some(PlayableId::Track(id)) => id,
        Some(PlayableId::Episode(_)) => return Err("Current playback not a track".into()),
        _ => return Err("Current playback has no `id`".into()),
    };

    let result = if current.saved {
        current.saved = false;
        app_state.emit_update(&app_handle);
        (*spotify_client)
            .current_user_saved_tracks_delete(vec![id])
            .await
    } else {
        current.saved = true;
        app_state.emit_update(&app_handle);
        (*spotify_client)
            .current_user_saved_tracks_add(vec![id])
            .await
    };

    if result.is_err() {
        *app_state = prev_app_state;
        app_state.emit_update(&app_handle);
        result?;
    }

    Ok(())
}

#[tauri::command]
pub async fn toggle_shuffle(
    app_handle: tauri::AppHandle,
    app_store: tauri::State<'_, AppStore>,
) -> Result<(), HandlerError> {
    let mut app_state = app_store.0.lock().await;

    {
        let prev_app_state = app_state.clone();
        let spotify_client = app_state.spotify_client.clone();
        let spotify_client = spotify_client.lock().await;

        app_state.shuffle = !app_state.shuffle;
        app_state.emit_update(&app_handle);
        let result = (*spotify_client)
            .shuffle(app_state.shuffle, app_state.device_id.as_deref())
            .await;

        if result.is_err() {
            *app_state = prev_app_state;
            app_state.emit_update(&app_handle);
            result?;
        }
    }

    app_state.get_current_playback().await?;

    Ok(())
}

async fn update(app_handle: Arc<Mutex<tauri::AppHandle>>) -> anyhow::Result<()> {
    let app_handle = (*app_handle).lock().await;

    let app_state = app_handle.state::<AppStore>().0.clone();
    let mut app_state = app_state.lock().await;

    let duration = app_state
        .curr
        .as_ref()
        .map(|c| c.duration_ms)
        .unwrap_or(u64::MAX);

    if app_state.last_playback_call.elapsed() >= AppState::PLAYBACK_CALL_BUFFER
        || app_state.progress_ms >= duration
    {
        app_state.get_current_playback().await?;
    }

    if app_state.curr.is_some()
        && app_state.last_seek_update.elapsed() >= AppState::SEEK_CALL_BUFFER
        && app_state.playing
    {
        let elapsed = app_state.last_seek_update.elapsed().as_millis();
        let diff = (elapsed / AppState::SEEK_CALL_BUFFER.as_millis()) as u64;
        let curr = app_state.curr.as_mut().unwrap();
        curr.progress_ms += diff;
        app_state.progress_ms = curr.progress_ms;
        app_state.last_seek_update = Instant::now();
    }

    app_state.emit_update(&app_handle);

    Ok(())
}

pub fn subscribe_to_event_loop(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let event_loop_handle = app_handle.state::<EventLoopHandle>();
    let app_handle_mutex = Arc::new(Mutex::new(app_handle.clone()));

    let mut event_loop_handle = event_loop_handle.0.lock().unwrap();
    *event_loop_handle = Some(tauri::async_runtime::spawn(async move {
        loop {
            let status = update(app_handle_mutex.clone()).await;
            #[cfg(debug_assertions)]
            if let Err(error) = status {
                println!("Error in event loop.\n\tError: {}", error);
            }
        }
    }));

    Ok(())
}

pub fn unsubscribe_to_event_loop(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let event_loop_handle = app_handle.state::<EventLoopHandle>();

    let event_loop_handle = event_loop_handle.0.lock().unwrap();

    if let Some(event_loop_handle) = &*event_loop_handle {
        event_loop_handle.abort();
    }

    Ok(())
}
