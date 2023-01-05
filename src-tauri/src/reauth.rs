use rspotify::{prelude::BaseClient, Token};
use tauri::Manager;
use tauri_plugin_store::{with_store, StoreCollection};
use thiserror::Error;

use crate::{
    scopes::get_scopes,
    state::{AppStore, STORE_PATH_BUF, STORE_TOKEN_KEY},
};

#[derive(Debug, Error)]
enum ReauthError {
    #[error("token error: {0}")]
    TokenError(String),
}

pub async fn reauth_spotify(app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
    let collection = app_handle.state::<StoreCollection>();

    let token_store = with_store(
        app_handle,
        collection.clone(),
        STORE_PATH_BUF.parse().unwrap(),
        |store| Ok(store.cache.get(&STORE_TOKEN_KEY.to_string()).cloned()),
    )
    .map_err(|o| ReauthError::TokenError(o.to_string()))?;

    let token_store =
        token_store.ok_or_else(|| ReauthError::TokenError("No Token Present".into()))?;

    let token = token_store
        .get("refresh_token")
        .ok_or_else(|| {
            ReauthError::TokenError("Token store doesn't contain a refresh token".into())
        })
        .map(|t| Token {
            refresh_token: serde_json::from_value::<String>(t.clone()).ok(),
            scopes: get_scopes(),
            ..Token::default()
        })?;

    let app_state = app_handle.state::<AppStore>().0.clone();
    let app_state = app_state.lock().await;
    let spotify_client = app_state.spotify_client.lock().await;
    *spotify_client.token.lock().await.unwrap() = Some(token);
    spotify_client.refresh_token().await?;

    let token = spotify_client.token.lock().await.unwrap();
    let serialized_token = serde_json::to_value(token.as_ref().unwrap()).unwrap();
    with_store(
        app_handle,
        collection,
        STORE_PATH_BUF.parse().unwrap(),
        |store| {
            Ok(store
                .cache
                .insert(STORE_TOKEN_KEY.to_string(), serialized_token))
        },
    )
    .map_err(|o| ReauthError::TokenError(o.to_string()))?;
    Ok(())
}
