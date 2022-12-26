use std::sync::Arc;

use rspotify::{AuthCodeSpotify, ClientResult};
use std::sync::Mutex as SyncMutex;
use tauri::async_runtime::{JoinHandle, Mutex};

pub struct SpotifyClient(pub Arc<Mutex<Option<AuthCodeSpotify>>>);

pub struct EventLoopHandle(pub SyncMutex<Option<JoinHandle<ClientResult<()>>>>);
