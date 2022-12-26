use std::sync::Arc;

use rspotify::model::{track, CurrentPlaybackContext, FullTrack, PlayableItem};
use rspotify::{model::AdditionalType, prelude::OAuthClient, AuthCodeSpotify};
use rspotify::{ClientError, ClientResult};
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_store::{with_store, StoreCollection};

use crate::redirect_uri::redirect_uri_web_server;
use crate::state::{
    EventLoopHandle, SpotifyClient, SpotifyClientInner, STORE_PATH_BUF, STORE_TOKEN_KEY,
};

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
    spotify_client: tauri::State<'_, SpotifyClient>,
) -> Result<(), String> {
    let mut spotify = spotify_client.0.lock().await;

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

async fn event_loop(
    app_handle: Arc<Mutex<tauri::AppHandle>>,
    spotify_client: SpotifyClientInner,
) -> ClientResult<()> {
    use AdditionalType::*;
    loop {
        let spotify_client = spotify_client.lock().await;

        if spotify_client.token.lock().await.unwrap().is_none() {
            continue;
        }

        let context = (*spotify_client)
            .current_playback(None, Some(vec![Track, Episode].iter()))
            .await;

        if let Ok(Some(context)) = context {
            let app_handle = (*app_handle).lock().await;

            let seriealized_context = serde_json::to_value(context.clone()).unwrap();
            let mut seriealized_context = seriealized_context.as_object().unwrap().to_owned();

            let liked = match &context.item {
                Some(PlayableItem::Track(FullTrack { id: Some(id), .. })) => {
                    match (*spotify_client)
                        .current_user_saved_tracks_contains(vec![id.clone()])
                        .await
                        .ok()
                        .and_then(|songs| songs.first().cloned())
                    {
                        Some(liked) => liked,
                        _ => false,
                    }
                }
                _ => false,
            };

            seriealized_context.insert("liked".to_string(), serde_json::to_value(liked).unwrap());

            app_handle
                .emit_all("playback", seriealized_context)
                .unwrap();
        }
    }
}

pub fn subscribe_to_event_loop(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let spotify_client = app_handle.state::<SpotifyClient>();
    let event_loop_handle = app_handle.state::<EventLoopHandle>();

    let spotify_client = spotify_client.0.clone();
    let app_handle_mutex = Arc::new(Mutex::new(app_handle.clone()));

    let mut event_loop_handle = event_loop_handle.0.lock().unwrap();
    *event_loop_handle = Some(tauri::async_runtime::spawn(async move {
        event_loop(app_handle_mutex, spotify_client)
            .await
            .expect("crash in event loop.")
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
