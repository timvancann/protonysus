use crate::player::{Library, PlayerControls, PlayerEvent, PlayerState, Track};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tauri::State;
use crate::library::modify_library;

#[tauri::command]
pub fn player_state(state: State<'_, Arc<Mutex<PlayerState>>>, controls: State<'_, PlayerControls>) -> Value {
    controls
        .inner()
        .player_event_sender
        .send(PlayerEvent::UpdateProgress)
        .unwrap();
    let state = state.lock().unwrap();
    json!(*state)
}

#[tauri::command]
pub fn pause(controls: State<'_, PlayerControls>) {
    controls
        .inner()
        .player_event_sender
        .send(PlayerEvent::Pause)
        .unwrap();
}

#[tauri::command]
pub fn select_track(track: Track, controls: State<'_, PlayerControls>) {
    controls
        .inner()
        .player_event_sender
        .send(PlayerEvent::NewTrack(track))
        .unwrap();
}

#[tauri::command]
pub fn play(controls: State<'_, PlayerControls>) {
    controls
        .inner()
        .player_event_sender
        .send(PlayerEvent::Play)
        .unwrap();
}

#[tauri::command]
pub fn stop(controls: State<'_, PlayerControls>) {
    controls
        .inner()
        .player_event_sender
        .send(PlayerEvent::Stop)
        .unwrap();
}

#[tauri::command(async, rename_all = "snake_case")]
pub fn upload_file(files: Vec<&str>,
                   library: State<'_, Arc<Mutex<Library>>>) -> Value
{
    modify_library(&library, files);
    
    let serialised = json!(*library.lock().unwrap());
    serialised
}


#[tauri::command]
pub fn next(
    state: State<'_, Arc<Mutex<PlayerState>>>,
    controls: State<'_, PlayerControls>,
    library: State<'_, Arc<Mutex<Library>>>,
) {
    let current_track = state.lock().unwrap().current_track.clone();
    let lib = library.lock().unwrap();
    let next_track = match current_track {
        Some(current_track) => {
            let current_track_index = lib.tracks.iter().position(|track| track == &current_track).unwrap();
            if current_track_index + 1 < lib.tracks.len() {
                Some(lib.tracks[current_track_index + 1].clone())
            } else {
                Some(lib.tracks[0].clone())
            }
        }
        None => None,
    };
    if let Some(track) = next_track {
        controls
            .inner()
            .player_event_sender
            .send(PlayerEvent::NewTrack(track))
            .unwrap();
    }
}

#[tauri::command]
pub fn previous(
    state: State<'_, Arc<Mutex<PlayerState>>>,
    controls: State<'_, PlayerControls>,
    library: State<'_, Arc<Mutex<Library>>>,
) {
    let current_track = state.lock().unwrap().current_track.clone();
    let lib = library.lock().unwrap();
    let previous_track = match current_track {
        Some(current_track) => {
            let current_track_index = lib.tracks.iter().position(|track| track == &current_track).unwrap();
            if current_track_index > 0 {
                Some(lib.tracks[current_track_index - 1].clone())
            } else {
                Some(lib.tracks[lib.tracks.len() - 1].clone())
            }
        }
        None => None,
    };
    if let Some(track) = previous_track {
        controls
            .inner()
            .player_event_sender
            .send(PlayerEvent::NewTrack(track))
            .unwrap();
    }
}
