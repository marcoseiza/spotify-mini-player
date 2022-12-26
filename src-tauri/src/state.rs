use std::sync::Arc;

use rspotify::AuthCodeSpotify;
use std::sync::Mutex as SyncMutex;
use tauri::async_runtime::{JoinHandle, Mutex};

pub type SpotifyClientInner = Arc<Mutex<AuthCodeSpotify>>;
pub struct SpotifyClient(pub SpotifyClientInner);

pub struct EventLoopHandle(pub SyncMutex<Option<JoinHandle<()>>>);

pub const STORE_PATH_BUF: &str = "store.bin";
pub const STORE_TOKEN_KEY: &str = "access_token";
