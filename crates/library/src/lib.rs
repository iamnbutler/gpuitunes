use gpui::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub fn format_playback_time(seconds: i32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TrackId(String);

impl Into<String> for TrackId {
    fn into(self) -> String {
        self.0
    }
}

fn track_id(title: String, artist: String, album: String) -> TrackId {
    let uuid = uuid::Uuid::new_v4();
    let id = format!("{}-{}-{}-{}", title, artist, album, uuid);
    TrackId(id)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTrack {
    title: String,
    artist: String,
    album: String,
    duration: i32,
    kind: String,
    date_added: String,
    plays: i32,
    track_number: u32,
    total_tracks: u32,
}

#[derive(Debug, Clone)]
pub struct Track {
    _id: TrackId,
    title: SharedString,
    artist: SharedString,
    album: SharedString,
    duration: i32,
    _kind: String,
    _date_added: String,
    plays: i32,
    track_number: u32,
    total_tracks: u32,
}

impl From<SerializableTrack> for Track {
    fn from(track: SerializableTrack) -> Self {
        let title = track.title.clone();
        let artist = track.artist.clone();
        let album = track.album.clone();

        Track {
            _id: track_id(title.clone(), artist.clone(), album.clone()),
            title: track.title.into(),
            artist: track.artist.into(),
            album: track.album.into(),
            duration: track.duration,
            _kind: track.kind,
            _date_added: track.date_added,
            plays: track.plays,
            track_number: track.track_number,
            total_tracks: track.total_tracks,
        }
    }
}

pub struct NowPlaying {
    current_track: Option<CurrentTrack>,
}

impl Default for NowPlaying {
    fn default() -> Self {
        NowPlaying {
            current_track: None,
        }
    }
}

impl NowPlaying {
    pub fn current_track(&self) -> Option<&CurrentTrack> {
        self.current_track.as_ref()
    }

    pub fn set_current_track(&mut self, current_track: Option<CurrentTrack>) {
        self.current_track = current_track;
    }
}

#[derive(Debug, Clone)]
pub struct CurrentTrack {
    track: Track,
    is_playing: bool,
    current_time: i32,
}

impl CurrentTrack {
    pub fn new(track: Track) -> Self {
        CurrentTrack {
            track,
            is_playing: false,
            current_time: 0,
        }
    }

    pub fn album(&self) -> SharedString {
        self.track.album.clone()
    }

    pub fn artist(&self) -> SharedString {
        self.track.artist.clone()
    }

    pub fn current_time(&self) -> i32 {
        self.current_time
    }

    pub fn duration(&self) -> i32 {
        self.track.duration
    }

    pub fn progress(&self) -> f32 {
        (self.current_time as f32 / self.duration() as f32).clamp(0., 1.)
    }

    pub fn time_remaining(&self) -> i32 {
        self.duration() - self.current_time()
    }

    pub fn title(&self) -> SharedString {
        self.track.title.clone()
    }

    pub fn track(&self) -> &Track {
        &self.track
    }

    pub fn track_number(&self) -> String {
        format!("{} of {}", self.track.track_number, self.track.total_tracks)
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn set_current_time(&mut self, time: i32) {
        self.current_time = time;
    }

    pub fn set_is_playing(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }

    pub fn set_track(&mut self, track: Track) {
        self.track = track;
    }

    pub fn set_plays(&mut self, plays: i32) {
        self.track.plays = plays;
    }

    pub fn increment_plays(&mut self) {
        self.track.plays += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLibrary {
    tracks: Vec<SerializableTrack>,
    columns: Vec<Column>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnKind {
    Playing,
    Title,
    Artist,
    Album,
    Duration,
    TrackNumber,
    Kind,
    DateAdded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    kind: ColumnKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f32>,
    enabled: bool,
}

fn _default_columns() -> Vec<Column> {
    vec![
        Column::new(ColumnKind::Playing),
        Column::new(ColumnKind::Title),
        Column::new(ColumnKind::Artist),
        Column::new(ColumnKind::Album),
        Column::new(ColumnKind::Duration),
        Column::new(ColumnKind::TrackNumber),
        Column::new(ColumnKind::Kind),
        Column::new(ColumnKind::DateAdded),
    ]
}

impl Column {
    pub fn new(kind: ColumnKind) -> Self {
        Column {
            kind,
            width: None,
            enabled: true,
        }
    }

    pub fn name(&self) -> String {
        match self.kind {
            ColumnKind::Playing => "".to_string(),
            ColumnKind::Title => "Name".to_string(),
            ColumnKind::Artist => "Artist".to_string(),
            ColumnKind::Album => "Album".to_string(),
            ColumnKind::Duration => "Time".to_string(),
            ColumnKind::TrackNumber => "Track Number".to_string(),
            ColumnKind::Kind => "Kind".to_string(),
            ColumnKind::DateAdded => "Date Added".to_string(),
        }
    }

    pub fn width(&self) -> f32 {
        self.width.unwrap_or(match self.kind {
            ColumnKind::Playing => 17.0,
            ColumnKind::Title => 300.0,
            ColumnKind::Artist => 150.0,
            ColumnKind::Album => 150.0,
            ColumnKind::Duration => 100.0,
            ColumnKind::TrackNumber => 50.0,
            ColumnKind::Kind => 100.0,
            ColumnKind::DateAdded => 150.0,
        })
    }

    pub fn set_width(&mut self, width: Option<f32>) {
        self.width = width;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

pub fn test_library_path() -> PathBuf {
    std::env::current_dir()
        .expect("Failed to get current directory")
        .join("library")
}

pub struct Library {
    _source: Option<PathBuf>,
    _tracks: HashMap<TrackId, Track>,
    _track_order: Vec<TrackId>,
    _columns: Vec<Column>,
    _scanning_task: Option<Task<()>>,
}

impl Default for Library {
    fn default() -> Self {
        Library {
            _source: None,
            _tracks: HashMap::new(),
            _track_order: Vec::new(),
            _columns: Vec::new(),
            _scanning_task: None,
        }
    }
}

impl Library {
    pub fn new(cx: &mut WindowContext, path: PathBuf) -> Model<Self> {
        // check and load dir

        cx.new_model(|_cx| Library {
            _source: Some(path),
            _tracks: HashMap::new(),
            _track_order: Vec::new(),
            _columns: Vec::new(),
            _scanning_task: None,
        })
    }
}

impl EventEmitter<Event> for Library {}

pub enum Event {}
