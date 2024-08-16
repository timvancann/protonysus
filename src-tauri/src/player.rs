use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum PlayState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Id3Tags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub album_art: Option<String>,
    pub track: Option<u32>,
}

impl Id3Tags {
    pub fn new() -> Self {
        Self {
            title: None,
            artist: None,
            album: None,
            year: None,
            album_art: None,
            track: None,
        }
    }
}

#[derive(Serialize, PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub struct Track {
    pub(crate) path: PathBuf,
    pub(crate) file_name: String,
    pub(crate) duration: u64,
    pub id3_tags: Id3Tags,
}

#[derive(Serialize)]
pub struct PlayerState {
    play_state: PlayState,
    pub(crate) current_track: Option<Track>,
    progress: u64,
}

#[derive(Serialize)]
pub struct Library {
    pub(crate) tracks: Vec<Track>,
}

impl Library {
    pub fn new() -> Self {
        Self { tracks: vec![] }
    }
}

impl PlayerState {
    pub(crate) fn new() -> Self {
        Self {
            play_state: PlayState::Stopped,
            current_track: None,
            progress: 0,
        }
    }
}

pub enum PlayerEvent {
    Play,
    Pause,
    Stop,
    NewTrack(Track),
    UpdateProgress,
}

pub struct PlayerControls {
    pub(crate) player_event_sender: Sender<PlayerEvent>,
    player_event_receiver: Arc<Mutex<Receiver<PlayerEvent>>>,
}

impl PlayerControls {
    pub fn new() -> Self {
        let (player_event_sender, player_controls_receiver) = channel::<PlayerEvent>();
        PlayerControls {
            player_event_sender,
            player_event_receiver: Arc::new(Mutex::new(player_controls_receiver)),
        }
    }

    pub fn init(&self, state: &Arc<Mutex<PlayerState>>) {
        let player_event_receiver = self.player_event_receiver.clone();
        let state = state.clone();

        thread::spawn(move || {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                if let Ok(res) = player_event_receiver.lock().unwrap().recv() {
                    match res {
                        PlayerEvent::Play => {
                            if state.lock().unwrap().current_track.is_none() {
                                continue;
                            }
                            state.lock().unwrap().play_state = PlayState::Playing;
                            sink.play();
                        }
                        PlayerEvent::Pause => {
                            if state.lock().unwrap().current_track.is_none() {
                                continue;
                            }
                            state.lock().unwrap().play_state = PlayState::Paused;
                            sink.pause();
                        }
                        PlayerEvent::Stop => {
                            state.lock().unwrap().play_state = PlayState::Stopped;
                            state.lock().unwrap().current_track = None;
                            sink.stop();
                        }
                        PlayerEvent::NewTrack(track) => {
                            let file = std::fs::File::open(&track.path).unwrap();
                            let source = Decoder::new(BufReader::new(file)).unwrap();

                            state.lock().unwrap().play_state = PlayState::Playing;
                            state.lock().unwrap().current_track = Some(track);

                            sink.clear();
                            sink.append(source);
                            sink.play();
                        }
                        PlayerEvent::UpdateProgress => {
                            let progress = sink.get_pos().as_millis() as u64;
                            state.lock().unwrap().progress = progress;
                        }
                    };
                }
            }
        });
    }
}

