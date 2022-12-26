use std::sync::Arc;

use rspotify::{
    model::AdditionalType, prelude::OAuthClient, scopes, AuthCodeSpotify, Credentials, OAuth,
};
use rspotify::{ClientError, ClientResult};
use tauri::async_runtime::Mutex;
use tauri::Manager;

use crate::redirect_uri::redirect_uri_web_server;
use crate::state::{EventLoopHandle, SpotifyClient};

pub async fn get_token_auto(spotify_oauth: &mut AuthCodeSpotify, port: u16) -> ClientResult<()> {
    match spotify_oauth.read_token_cache(true).await {
        Ok(Some(_)) => Ok(()),
        _ => match redirect_uri_web_server(spotify_oauth, port) {
            Ok(url) => {
                let code = spotify_oauth.parse_response_code(&url).ok_or_else(|| {
                    ClientError::Cli("unable to parse the response code".to_string())
                })?;
                spotify_oauth.request_token(&code).await
            }
            Err(()) => Ok(()),
        },
    }
}

#[tauri::command]
pub async fn login_spotify(spotify_client: tauri::State<'_, SpotifyClient>) -> Result<(), String> {
    let creds = Credentials::from_env().unwrap();
    // Using every possible scope
    let scopes = scopes!(
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "user-modify-playback-state"
    );
    let oauth = OAuth::from_env(scopes).unwrap();

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    get_token_auto(&mut spotify, 8585).await.unwrap();

    let mut spotify_client = spotify_client.0.lock().await;
    *spotify_client = Some(spotify);

    Ok(())
}

async fn event_loop(
    app_handle: Arc<Mutex<tauri::AppHandle>>,
    spotify_client: Arc<Mutex<Option<AuthCodeSpotify>>>,
) -> ClientResult<()> {
    use AdditionalType::*;
    loop {
        let spotify_client = spotify_client.lock().await;

        let context = match (*spotify_client).as_ref() {
            Some(client) => {
                client
                    .current_playback(None, Some(vec![Track, Episode].iter()))
                    .await?
            }
            None => None,
        };

        let app_handle = (*app_handle).lock().await;
        app_handle.emit_all("playback", context).unwrap();
    }
}

pub fn subscribe_to_event_loop(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let spotify_client = app_handle.state::<SpotifyClient>();
    let event_loop_handle = app_handle.state::<EventLoopHandle>();

    let spotify_client_arc = spotify_client.0.clone();

    let app_handle_mutex = Arc::new(Mutex::new(app_handle.clone()));

    let mut event_loop_handle = event_loop_handle.0.lock().unwrap();
    *event_loop_handle = Some(tauri::async_runtime::spawn(async move {
        event_loop(app_handle_mutex, spotify_client_arc).await
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
