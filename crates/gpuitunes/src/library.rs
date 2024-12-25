use gpui::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct PlaybackTime(i32);

impl PlaybackTime {
    pub fn format(&self) -> String {
        let minutes = self.0 / 60;
        let seconds = self.0 % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl From<i32> for PlaybackTime {
    fn from(seconds: i32) -> Self {
        PlaybackTime(seconds)
    }
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
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: i32,
    pub kind: String,
    pub date_added: String,
    pub plays: i32,
    pub track_number: u32,
    pub total_tracks: u32,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: TrackId,
    pub title: SharedString,
    pub artist: SharedString,
    pub album: SharedString,
    pub duration: PlaybackTime,
    pub kind: String,
    pub date_added: String,
    pub plays: i32,
    pub track_number: u32,
    pub total_tracks: u32,
}

impl From<SerializableTrack> for Track {
    fn from(track: SerializableTrack) -> Self {
        let title = track.title.clone();
        let artist = track.artist.clone();
        let album = track.album.clone();

        Track {
            id: track_id(title.clone(), artist.clone(), album.clone()),
            title: track.title.into(),
            artist: track.artist.into(),
            album: track.album.into(),
            duration: track.duration.into(),
            kind: track.kind,
            date_added: track.date_added,
            plays: track.plays,
            track_number: track.track_number,
            total_tracks: track.total_tracks,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentTrack {
    pub track: Track,
    pub current_time: PlaybackTime,
}

impl CurrentTrack {
    pub fn new(track: Track) -> Self {
        CurrentTrack {
            track,
            current_time: PlaybackTime(0),
        }
    }

    pub fn title(&self) -> SharedString {
        self.track.title.clone()
    }

    pub fn artist(&self) -> SharedString {
        self.track.artist.clone()
    }

    pub fn album(&self) -> SharedString {
        self.track.album.clone()
    }

    pub fn duration(&self) -> PlaybackTime {
        self.track.duration
    }

    pub fn time_remaining(&self) -> PlaybackTime {
        let time = self.track.duration.0 - self.current_time.0;
        time.into()
    }

    pub fn percent_complete(&self) -> f32 {
        (self.current_time.0 as f32 / self.track.duration.0 as f32).clamp(0.0, 1.0)
    }

    pub fn track_number(&self) -> String {
        format!("{} of {}", self.track.track_number, self.track.total_tracks)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLibrary {
    pub tracks: Vec<SerializableTrack>,
    pub columns: Vec<Column>,
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
    pub kind: ColumnKind,
    pub width: Option<f32>,
    pub enabled: bool,
}

fn default_columns() -> Vec<Column> {
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

    pub fn set_width(&mut self, width: Option<f32>) {
        self.width = width;
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

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Clone)]
pub struct Library {
    pub tracks: HashMap<TrackId, Track>,
    pub track_order: Vec<TrackId>,
    pub columns: Vec<Column>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            tracks: HashMap::new(),
            track_order: Vec::new(),
            columns: Vec::new(),
        }
    }

    pub fn columns(&self) -> Vec<Column> {
        self.columns.clone()
    }

    pub fn set_columns(&mut self, columns: Vec<Column>) {
        self.columns = columns;
    }

    pub fn sort_by_column(&mut self, column: ColumnKind) {
        match column {
            ColumnKind::Playing => (),
            ColumnKind::Title => self.track_order.sort_by(|a, b| {
                let track_a = self.tracks.get(a).unwrap();
                let track_b = self.tracks.get(b).unwrap();
                track_a.title.cmp(&track_b.title)
            }),
            ColumnKind::Artist => self.sort_by_artist(),
            ColumnKind::Album => self.track_order.sort_by(|a, b| {
                let track_a = self.tracks.get(a).unwrap();
                let track_b = self.tracks.get(b).unwrap();
                track_a
                    .album
                    .cmp(&track_b.album)
                    .then(track_a.artist.cmp(&track_b.artist))
                    .then(track_a.track_number.cmp(&track_b.track_number))
            }),
            ColumnKind::Duration => self.track_order.sort_by(|a, b| {
                let track_a = self.tracks.get(a).unwrap();
                let track_b = self.tracks.get(b).unwrap();
                track_a.duration.0.cmp(&track_b.duration.0)
            }),
            ColumnKind::TrackNumber => self.track_order.sort_by(|a, b| {
                let track_a = self.tracks.get(a).unwrap();
                let track_b = self.tracks.get(b).unwrap();
                track_a.track_number.cmp(&track_b.track_number)
            }),
            ColumnKind::Kind => self.track_order.sort_by(|a, b| {
                let track_a = self.tracks.get(a).unwrap();
                let track_b = self.tracks.get(b).unwrap();
                track_a.kind.cmp(&track_b.kind)
            }),
            ColumnKind::DateAdded => self.track_order.sort_by(|a, b| {
                let track_a = self.tracks.get(a).unwrap();
                let track_b = self.tracks.get(b).unwrap();
                track_a.date_added.cmp(&track_b.date_added)
            }),
        }
    }
}

impl Library {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data");
        path.push("library.json");

        let file = std::fs::File::open(path)?;
        let serializable_library: SerializableLibrary = serde_json::from_reader(file)?;

        let tracks: HashMap<TrackId, Track> = serializable_library
            .tracks
            .into_iter()
            .map(|track| {
                let track: Track = track.into();
                (track.id.clone(), track)
            })
            .collect();

        let ordered_keys: Vec<TrackId> = tracks
            .clone()
            .values()
            .map(|track| track.id.clone())
            .collect();

        let columns = if serializable_library.columns.is_empty() {
            default_columns()
        } else {
            serializable_library.columns.clone()
        };

        let mut library = Library {
            tracks,
            track_order: ordered_keys,
            columns,
        };

        library.sort_by_artist();

        Ok(library)
    }

    fn sort_by_artist(&mut self) {
        self.track_order.sort_by(|a, b| {
            let track_a = self.tracks.get(a).unwrap();
            let track_b = self.tracks.get(b).unwrap();
            track_a
                .artist
                .cmp(&track_b.artist)
                .then(track_a.album.cmp(&track_b.album))
                .then(track_a.track_number.cmp(&track_b.track_number))
        });
    }
}
