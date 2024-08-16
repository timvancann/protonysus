#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod player;
mod endpoints;
mod tagging;
mod library;

use crate::endpoints::{next, pause, play, player_state, previous, select_track, stop, upload_file};
use crate::player::{Library, PlayerControls, PlayerState};
use std::sync::{Arc, Mutex};
use tauri::{generate_context, Manager, State};

fn main() {
    let controls = PlayerControls::new();
    let player = Arc::new(Mutex::new(PlayerState::new()));
    let library = Arc::new(Mutex::new(Library::new()));

    tauri::Builder::default()
        .manage(controls)
        .manage(player)
        .manage(library)
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            let state: State<Arc<Mutex<PlayerState>>> = app.state();
            let controls: State<PlayerControls> = app.state();
            controls.init(&state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            upload_file,
            pause,
            stop,
            play,
            player_state,
            select_track,
            next,
            previous
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}