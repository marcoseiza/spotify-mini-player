#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSWindowTitleVisibility};
use cocoa::base::id;
use reauth::reauth_spotify;

use rspotify::{AuthCodeSpotify, Credentials, OAuth};
use scopes::get_scopes;
use tauri_plugin_store::{with_store, PluginBuilder, StoreBuilder, StoreCollection};

use std::sync::Mutex as SyncMutex;
use tauri::async_runtime::Mutex;
use tauri::{
    ActivationPolicy, AppHandle, GlobalWindowEvent, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, Window, WindowBuilder, WindowEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

mod handlers;
mod helpers;
mod reauth;
mod redirect_uri;
mod scopes;
mod state;

use handlers::*;
use state::*;

fn create_tray_window(app_handle: &AppHandle) -> tauri::Result<Window> {
    let window = WindowBuilder::new(
        app_handle,
        "main",
        tauri::WindowUrl::App("index.html".into()),
    )
    .fullscreen(false)
    .inner_size(200., 340.)
    .resizable(false)
    .title("spotify mini player")
    .transparent(true)
    .decorations(true)
    .visible(false)
    .always_on_top(true)
    .build()
    .unwrap();
    window.move_window(Position::TrayBottomCenter).unwrap();

    #[cfg(target_os = "macos")]
    {
        apply_vibrancy(
            &window,
            NSVisualEffectMaterial::Popover,
            Some(NSVisualEffectState::Active),
            Some(9.0),
        )
        .unwrap();

        window
            .with_webview(|webview| unsafe {
                use NSWindowTitleVisibility::*;
                let ns_window: id = webview.ns_window();
                ns_window.setTitleVisibility_(NSWindowTitleHidden);
                ns_window.setStyleMask_(NSWindowStyleMask::NSFullSizeContentViewWindowMask);
            })
            .unwrap();
    }

    window.show().unwrap();
    Ok(window)
}

fn handle_on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);
    if let SystemTrayEvent::LeftClick { .. } = event {
        match app.get_window("main") {
            Some(window) => {
                if let Ok(true) = window.is_visible() {
                    window.close().unwrap();
                    unsubscribe_to_event_loop(app).unwrap();
                }
            }
            None => {
                create_tray_window(app).unwrap();
                subscribe_to_event_loop(app).unwrap();
                let mutex_app = Box::new(Mutex::new(app.clone()));
                tauri::async_runtime::spawn(async move {
                    let app = mutex_app.lock().await;
                    reauth_spotify(&app).await.expect("Reauth Error");
                    let app_state = app.state::<AppStore>().0.clone();
                    let mut app_state = app_state.lock().await;
                    app_state.get_current_playback().await
                });
            }
        }
    }
}

fn handle_on_window_event(event: GlobalWindowEvent) {
    use WindowEvent::*;
    if let Focused(false) = event.event() {
        #[cfg(not(debug_assertions))]
        {
            let window = event.window();
            let app_handle = window.app_handle();
            window.close().unwrap();
            unsubscribe_to_event_loop(&app_handle).unwrap();
        }
    }
}

fn main() {
    env_logger::init();

    let creds = Credentials::from_env().unwrap();
    let scopes = get_scopes();
    let oauth = OAuth::from_env(scopes).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let store = StoreBuilder::new(STORE_PATH_BUF.parse().unwrap()).build();

    tauri::Builder::default()
        .plugin(PluginBuilder::default().store(store).build())
        .system_tray(SystemTray::new().with_menu(SystemTrayMenu::new()))
        .plugin(tauri_plugin_positioner::init())
        .on_system_tray_event(handle_on_system_tray_event)
        .on_window_event(handle_on_window_event)
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            Ok(())
        })
        .manage(EventLoopHandle(SyncMutex::new(None)))
        .manage(AppStore(Arc::new(Mutex::new(AppState {
            spotify_client: Arc::new(Mutex::new(spotify)),
            ..AppState::default()
        }))))
        .invoke_handler(tauri::generate_handler![
            login_spotify,
            next_track,
            prev_track,
            play_pause,
            toggle_saved,
            toggle_shuffle
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                let collection = app_handle.state::<StoreCollection>();
                with_store(
                    app_handle,
                    collection,
                    STORE_PATH_BUF.parse().unwrap(),
                    |store| store.save(app_handle),
                )
                .unwrap();
                api.prevent_exit();
            }
        });
}
