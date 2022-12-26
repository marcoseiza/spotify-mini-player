#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSWindowTitleVisibility};
use cocoa::base::id;

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
mod redirect_uri;
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
    .inner_size(200., 320.)
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
            Some(window) => match window.is_visible() {
                Ok(true) => {
                    window.close().unwrap();
                    unsubscribe_to_event_loop(app).unwrap();
                }
                _ => todo!(),
            },
            None => {
                create_tray_window(app).unwrap();
                subscribe_to_event_loop(app).unwrap();
            }
        }
    }
}

fn handle_on_window_event(event: GlobalWindowEvent) {
    if let WindowEvent::Focused(false) = event.event() {
        #[cfg(not(debug_assertions))]
        {
            let window = event.window();
            let app_handle = window.app_handle();
            window.close().unwrap();
            unsubscribe_to_event_loop(&app_handle).unwrap();
        }
    };
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(SystemTrayMenu::new()))
        .plugin(tauri_plugin_positioner::init())
        .on_system_tray_event(handle_on_system_tray_event)
        .on_window_event(handle_on_window_event)
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            Ok(())
        })
        .manage(SpotifyClient(Arc::new(Mutex::new(None))))
        .manage(EventLoopHandle(SyncMutex::new(None)))
        .invoke_handler(tauri::generate_handler![login_spotify])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
