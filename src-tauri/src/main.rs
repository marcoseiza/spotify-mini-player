#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSWindowTitleVisibility};
use cocoa::base::id;

use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, ClientError, ClientResult, Credentials, OAuth};
use tauri::{
    ActivationPolicy, AppHandle, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    Window, WindowBuilder, WindowEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

mod redirect_uri;
use redirect_uri::*;

/// get token automatically with local webserver
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

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(SystemTrayMenu::new()))
        .plugin(tauri_plugin_positioner::init())
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            if let SystemTrayEvent::LeftClick { .. } = event {
                match app.get_window("main") {
                    Some(window) => match window.is_visible() {
                        Ok(true) => {
                            window.close().unwrap();
                        }
                        _ => todo!(),
                    },
                    None => {
                        create_tray_window(app).unwrap();
                    }
                }
            }
        })
        .on_window_event(|event| {
            #[cfg(not(debug_assertions))]
            let window = event.window();
            if let WindowEvent::Focused(false) = event.event() {
                #[cfg(not(debug_assertions))]
                window.close().unwrap();
            };
        })
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);

            let creds = Credentials::from_env().unwrap();
            // Using every possible scope
            let scopes = scopes!(
                "user-read-email",
                "user-read-private",
                "user-top-read",
                "user-read-recently-played",
                "user-follow-read",
                "user-library-read",
                "user-read-currently-playing",
                "user-read-playback-state",
                "user-read-playback-position",
                "playlist-read-collaborative",
                "playlist-read-private",
                "user-follow-modify",
                "user-library-modify",
                "user-modify-playback-state",
                "playlist-modify-public",
                "playlist-modify-private",
                "ugc-image-upload"
            );
            let oauth = OAuth::from_env(scopes).unwrap();

            let mut spotify = AuthCodeSpotify::new(creds, oauth);

            tauri::async_runtime::spawn(async move {
                get_token_auto(&mut spotify, 8585).await.unwrap();
                let token = spotify.token.lock().await.unwrap();
                println!("Access token: {}", &token.as_ref().unwrap().access_token);
                println!(
                    "Refresh token: {}",
                    token.as_ref().unwrap().refresh_token.as_ref().unwrap()
                );
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
