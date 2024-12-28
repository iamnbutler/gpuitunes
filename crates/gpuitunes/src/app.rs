#![allow(unused, dead_code)]

use std::{collections::HashMap, sync::Arc, time::Duration};

use gpui::*;
use serde::{Deserialize, Serialize};

const UPDATE_INTERVAL: Duration = Duration::from_millis(250);

pub struct AppState {
    pending_update: Option<Task<()>>,
}

pub struct UpdateTriggered;

impl AppState {
    pub fn new(cx: &mut AppContext) -> Self {
        AppState {
            pending_update: None,
        }
    }

    fn init_update(&mut self, cx: &mut ModelContext<Self>) {
        if self.pending_update.is_none() {
            self.pending_update = Some(self.start_updates(cx));
        }
    }

    fn start_updates(&self, cx: &mut ModelContext<Self>) -> Task<()> {
        cx.spawn(|this, mut cx| async move {
            loop {
                cx.background_executor().timer(UPDATE_INTERVAL).await;
                this.update(&mut cx, |this, cx| {
                    cx.emit(UpdateTriggered);
                })
                .ok();
            }
        })
    }
}

impl EventEmitter<UpdateTriggered> for AppState {}

pub struct Sidebar {
    window: WeakView<AppWindow>,
    library: Model<Library>,
    now_playing: Model<NowPlaying>,
}

impl Sidebar {
    pub fn new(
        window: WeakView<AppWindow>,
        library: Model<Library>,
        now_playing: Model<NowPlaying>,
    ) -> Self {
        Sidebar {
            window,
            library,
            now_playing,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

struct LibraryView {
    window: WeakView<AppWindow>,
    library: Model<Library>,
    now_playing: Model<NowPlaying>,
    focus_handle: FocusHandle,
}

impl LibraryView {
    pub fn new(
        window: WeakView<AppWindow>,
        library: Model<Library>,
        now_playing: Model<NowPlaying>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        LibraryView {
            window,
            library,
            now_playing,
            focus_handle,
        }
    }

    pub fn focus_handle(&mut self) {
        self.focus_handle.clone();
    }
}

impl Render for LibraryView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

impl FocusableView for LibraryView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct StatusBar {
    window: WeakView<AppWindow>,
    library: Model<Library>,
}

impl StatusBar {
    pub fn new(window: WeakView<AppWindow>, library: Model<Library>) -> Self {
        StatusBar { window, library }
    }
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

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
    id: TrackId,
    title: SharedString,
    artist: SharedString,
    album: SharedString,
    duration: i32,
    kind: String,
    date_added: String,
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
            id: track_id(title.clone(), artist.clone(), album.clone()),
            title: track.title.into(),
            artist: track.artist.into(),
            album: track.album.into(),
            duration: track.duration,
            kind: track.kind,
            date_added: track.date_added,
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

#[derive(Debug, Clone)]
pub struct Library {
    pub tracks: HashMap<TrackId, Track>,
    pub track_order: Vec<TrackId>,
    pub columns: Vec<Column>,
}

pub struct AppWindow {
    weak_self: WeakView<Self>,
    sidebar: View<Sidebar>,
    // For now is just the library, but could
    // be a slot for any activated view
    active_view: View<LibraryView>,
    status_bar: View<StatusBar>,
    library: Model<Library>,
    now_playing: Model<NowPlaying>,
    app_state: Arc<AppState>,
    _subscriptions: Vec<Subscription>,
    // _schedule_serialize: Option<Task<()>>,
}

impl AppWindow {
    pub fn new(
        library: Model<Library>,
        app_state: Arc<AppState>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        // Watch for changes to the library, update the ui when they occur
        cx.observe(&library, |_, _, cx| cx.notify()).detach();
        // cx.subscribe(&library, move |this, _, event, cx| {
        //     match event {
        //         Event::LibraryUpdated => {
        //             // todo!(): something
        //         }
        //         _ => {}
        //     }
        // });

        // Ensure _something_ always has focus
        cx.on_focus_lost(|this, cx| {
            let focus_handle = this.focus_handle(cx);
            cx.focus(&focus_handle);
        })
        .detach();

        let weak_handle = cx.view().downgrade();

        let app_state = Arc::new(AppState::new(cx));

        let now_playing = cx.new_model(|_| NowPlaying::default());

        let sidebar = cx.new_view(|_cx| {
            Sidebar::new(weak_handle.clone(), library.clone(), now_playing.clone())
        });
        let library_view = cx.new_view(|cx| {
            LibraryView::new(
                weak_handle.clone(),
                library.clone(),
                now_playing.clone(),
                cx,
            )
        });
        let status_bar = cx.new_view(|_cx| StatusBar::new(weak_handle.clone(), library.clone()));

        AppWindow {
            weak_self: weak_handle,
            sidebar,
            active_view: library_view,
            status_bar,
            library,
            now_playing,
            app_state,
            _subscriptions: Vec::new(),
        }
    }
}

impl AppWindow {
    pub fn app_state(&self) -> &Arc<AppState> {
        &self.app_state
    }

    pub fn library(&self) -> &Model<Library> {
        &self.library
    }
}

impl FocusableView for AppWindow {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.active_view.focus_handle(cx)
    }
}

impl Render for AppWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // This should be more like 4.0, but later macOS versions have
        // a higher default window border radius
        let window_rounding = px(10.0);

        div()
            .id("gpuitunes-window")
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .rounded(window_rounding)
            // .relative()
            .bg(rgb(0xFEFFFF))
            .size_full()
            .font_family("Helvetica")
            .line_height(px(14.))
            .text_color(rgb(0x0F1219))
            .text_size(px(14.))
            .child("App Window")
    }
}

impl EventEmitter<Event> for AppWindow {}

pub enum Event {
    PlaybackStarted,
    PlaybackPaused,
    PlaybackStopped,
    LibraryUpdated,
}
