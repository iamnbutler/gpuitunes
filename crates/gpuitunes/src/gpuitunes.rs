#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use gpui::*;
use prelude::FluentBuilder as _;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;

fn h_stack() -> Div {
    div().flex().items_center()
}

fn v_stack() -> Div {
    div().flex().flex_col()
}

#[derive(IntoElement)]
struct Spacer {
    width: Option<Length>,
    height: Option<Length>,
    grow: bool,
}

impl Spacer {
    fn new() -> Self {
        Spacer {
            width: None,
            height: None,
            grow: false,
        }
    }

    fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    fn grow(mut self) -> Self {
        self.grow = true;
        self
    }
}

impl RenderOnce for Spacer {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let width = self.width.unwrap_or(Length::Auto);
        let height = self.height.unwrap_or(Length::Auto);

        div()
            .w(width)
            .h(height)
            .when(self.grow, |div| div.flex_grow())
            .child("")
    }
}

fn spacer() -> Spacer {
    Spacer::new()
}

fn circle(size: impl Into<DefiniteLength>) -> Div {
    div().size(size.into()).flex_none().rounded_full()
}

fn vertical_linear_gradient(start: impl Into<Hsla>, stop: impl Into<Hsla>) -> Background {
    let start = linear_color_stop(start, 0.0);
    let end = linear_color_stop(stop, 1.0);

    gpui::linear_gradient(180.0, start, end)
}

#[derive(Debug, Clone, Copy)]
struct PlaybackTime(i32);

impl PlaybackTime {
    fn format(&self) -> String {
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
struct TrackId(String);

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
struct SerializableTrack {
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
struct Track {
    id: TrackId,
    title: SharedString,
    artist: SharedString,
    album: SharedString,
    duration: PlaybackTime,
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
struct CurrentTrack {
    track: Track,
    current_time: PlaybackTime,
}

impl CurrentTrack {
    fn new(track: Track) -> Self {
        CurrentTrack {
            track,
            current_time: PlaybackTime(0),
        }
    }

    fn title(&self) -> SharedString {
        self.track.title.clone()
    }

    fn artist(&self) -> SharedString {
        self.track.artist.clone()
    }

    fn album(&self) -> SharedString {
        self.track.album.clone()
    }

    fn duration(&self) -> PlaybackTime {
        self.track.duration
    }

    fn time_remaining(&self) -> PlaybackTime {
        let time = self.track.duration.0 - self.current_time.0;
        time.into()
    }

    fn percent_complete(&self) -> f32 {
        (self.current_time.0 as f32 / self.track.duration.0 as f32) * 100.0
    }

    fn track_number(&self) -> String {
        format!("{} of {}", self.track.track_number, self.track.total_tracks)
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

    pub fn set_width(&mut self, width: Option<f32>) {
        self.width = width;
    }

    pub fn width(&self) -> f32 {
        self.width.unwrap_or(match self.kind {
            ColumnKind::Playing => 17.0,
            ColumnKind::Title => 200.0,
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
    tracks: HashMap<TrackId, Track>,
    track_order: Vec<TrackId>,
    columns: Vec<Column>,
}

impl Library {
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

struct AppState {
    current_track: CurrentTrack,
    library: Arc<Library>,
    sidebar_width: Option<f32>,
}

impl AppState {
    fn new(_cx: &mut ModelContext<Self>) -> Self {
        let default_track_base = SerializableTrack {
            title: "Feel Good Inc.".into(),
            artist: "Gorillaz".into(),
            album: "Demon Days".into(),
            duration: 120,
            kind: "MPEG audio file".into(),
            date_added: "2005-05-09".into(),
            plays: 34,
            track_number: 6,
            total_tracks: 15,
        };

        let default_track: Track = default_track_base.into();

        let library = match Library::load() {
            Ok(lib) => lib,
            Err(e) => {
                eprintln!("Failed to load library: {}", e);
                Library {
                    tracks: HashMap::new(),
                    track_order: Vec::new(),
                    columns: Vec::new(),
                }
            }
        };

        AppState {
            current_track: CurrentTrack::new(default_track),
            library: Arc::new(library),
            sidebar_width: None,
        }
    }

    fn update_library<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Library),
    {
        let mut library = (*self.library).clone();
        f(&mut library);
        self.library = Arc::new(library);
    }

    fn columns(&self) -> Vec<Column> {
        self.library.columns()
    }
}

struct Sidebar {
    state: Model<AppState>,
}

impl Sidebar {
    fn new(state: Model<AppState>) -> Self {
        Sidebar {
            state: state.clone(),
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

struct LibraryContent {
    state: Model<AppState>,
}

impl LibraryContent {
    fn new(state: Model<AppState>) -> Self {
        LibraryContent {
            state: state.clone(),
        }
    }

    fn render_entry(
        &mut self,
        ix: usize,
        id: &TrackId,
        is_selected: bool,
        track: &Track,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let id: String = id.clone().into();
        let is_odd = ix % 2 != 0;
        let library = self.state.read(cx).library.clone();
        let columns = library.columns();

        let mut row = h_stack()
            .id(ElementId::Name(id.into()))
            .when(is_odd, |div| div.bg(rgb(0xF0F0F0)))
            .when(is_selected, |div| div.bg(rgb(0xD0D0D0)))
            .min_w_full()
            .h(px(16.))
            .overflow_hidden()
            .text_size(px(12.));

        for column in columns.iter().filter(|c| c.enabled()) {
            let this = div().w(px(column.width()));

            let column_content = match column.kind {
                ColumnKind::Playing => this,
                ColumnKind::Title => this.child(track.title.clone()),
                ColumnKind::Artist => this.child(track.artist.clone()),
                ColumnKind::Album => this.child(track.album.clone()),
                ColumnKind::Duration => this.child(track.duration.format()),
                ColumnKind::TrackNumber => this.child(format!("{}", track.track_number)),
                ColumnKind::Kind => this.child(track.kind.clone()),
                ColumnKind::DateAdded => this.child(track.date_added.clone()),
            };

            row = row.child(
                div()
                    .overflow_hidden()
                    .w(px(column.width()))
                    .mr(px(6.))
                    .border_r_1()
                    .border_color(rgb(0xD9D9D9))
                    .h_full()
                    .child(column_content),
            );
        }

        row
    }

    fn render_column_header(
        &self,
        column: Column,
        state: Model<AppState>,
        _cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let column_kind = column.kind.clone();
        div()
            .id(ElementId::Name(format!("column-{}", column.name()).into()))
            .w(px(column.width()))
            .h_full()
            .flex()
            .items_center()
            .mr(px(6.))
            .border_r_1()
            .border_color(rgb(0xD9D9D9))
            .overflow_hidden()
            .child(div().text_size(px(11.)).child(column.name()))
            // .on_mouse_down(MouseButton::Left, |_, cx| cx.prevent_default())
            .on_click(move |e, cx| {
                println!("{:?}", e);
                cx.stop_propagation();
                println!("{:?}", e);

                let state = state.clone();
                let column_kind = column_kind.clone();

                println!("Clicked on column: {}", column.name());
                state.update(cx, |state, cx| {
                    state.update_library(|library| {
                        println!("Sorting by column: {:?}", column_kind);
                        library.sort_by_column(column_kind.clone());
                    });
                    cx.notify();
                });
            })
    }

    fn render_column_headers(
        &self,
        state: Model<AppState>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let columns = state.read(cx).columns();
        h_stack().min_w_full().h_full().children(
            columns
                .iter()
                .filter(|c| c.enabled())
                .map(|column| self.render_column_header(column.clone(), state.clone(), cx)),
        )
    }
}

impl Render for LibraryContent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.state.clone();
        let app_state = self.state.read(cx);
        let library = &app_state.library;
        let item_count = library.track_order.len();

        let list = uniform_list(cx.view().clone(), "library_content", item_count, {
            let library = library.clone();
            move |library_content, range, cx| {
                let mut items = Vec::with_capacity(range.end - range.start);
                for (ix, track_id) in library.track_order[range.start..range.end]
                    .iter()
                    .enumerate()
                {
                    if let Some(track) = library.tracks.get(track_id) {
                        items.push(library_content.render_entry(ix, track_id, false, track, cx));
                    }
                }
                items
            }
        })
        .size_full()
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained);

        v_stack()
            .flex_grow()
            .size_full()
            .child(
                h_stack()
                    .flex_shrink_0()
                    .w_full()
                    .h(px(17.))
                    .bg(rgb(0xF0F0F0))
                    .border_b_1()
                    .border_color(rgb(0xC0C0C0))
                    .child(self.render_column_headers(state.clone(), cx)),
            )
            .child(
                div()
                    .id("library-list-container")
                    .size_full()
                    .flex_grow()
                    .overflow_hidden()
                    .child(list.into_any_element()),
            )
    }
}

struct Footer {
    state: Model<AppState>,
}

impl Footer {
    fn new(state: Model<AppState>) -> Self {
        Footer {
            state: state.clone(),
        }
    }
}

impl Render for Footer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .h(px(36.))
            .border_t_1()
            .border_color(rgb(0x414141))
            .bg(vertical_linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
            .child(
                h_stack()
                    .ml(px(10.))
                    .child(
                        div()
                            .id("test-button")
                            .size(px(24.))
                            .bg(gpui::red())
                            .active(|this| this.opacity(0.8))
                            .on_click(cx.listener(move |this, event, cx| {
                                println!("{:?}", event);
                                cx.notify();
                            })),
                    )
                    .child(div().size(px(24.)).bg(gpui::blue())),
            )
            .child(div().text_size(px(12.)).child(format!(
                "{} tracks",
                self.state.read(cx).library.tracks.len()
            )))
            .child(
                h_stack()
                    .mr(px(10.))
                    .child(div().size(px(24.)).bg(gpui::green()))
                    .child(div().size(px(24.)).bg(gpui::yellow())),
            )
    }
}

struct TitleBar {
    state: Model<AppState>,
}

impl TitleBar {
    fn new(state: Model<AppState>) -> Self {
        TitleBar {
            state: state.clone(),
        }
    }
}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let traffic_lights_width = px(72.);
        let second_row_side = px(250.);

        v_stack()
            .w_full()
            .bg(vertical_linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
            .border_b_1()
            .border_color(rgb(0x414141))
            .child(
                h_stack()
                    .id("title-bar")
                    .h(px(20.))
                    .flex_none()
                    .w_full()
                    .justify_between()
                    .child(self.render_traffic_lights(traffic_lights_width))
                    .child(div().child("gpuiTunes"))
                    .child(spacer().width(traffic_lights_width)),
            )
            .child(
                div()
                    .flex()
                    .items_start()
                    .h(px(54.))
                    .flex_grow()
                    .w_full()
                    .gap(px(10.))
                    .child(
                        h_stack()
                            .w(second_row_side)
                            .h_full()
                            .child(spacer().width(px(27.)))
                            .child(self.render_playback_buttons(cx))
                            .child(self.render_volume_controls()),
                    )
                    .child(
                        h_stack()
                            .flex_grow()
                            .justify_center()
                            .child(self.render_now_playing(cx)),
                    )
                    .child(
                        div()
                            .h_full()
                            .flex_none()
                            .justify_center()
                            .w(second_row_side)
                            .child(
                                h_stack()
                                    .w(px(38.))
                                    .justify_center()
                                    .flex_none()
                                    .child(self.render_browse()),
                            ),
                    ),
            )
    }
}

impl TitleBar {
    fn render_traffic_light(&self) -> impl IntoElement {
        circle(px(14.))
            .mt(px(4.))
            .mb(px(2.))
            .rounded_full()
            .overflow_hidden()
            .p_px()
            // C5C5C5, BEBEBE, B8B6B7, AFAFAF, A7A7A7, 9F9DA0, 969696
            .bg(vertical_linear_gradient(rgb(0x101010), rgb(0x95999C)))
            .shadow(smallvec![BoxShadow {
                color: hsla(0.0, 1., 1., 0.36),
                offset: point(px(0.), px(1.)),
                blur_radius: px(1.),
                spread_radius: px(1.),
            }])
            .child(
                circle(px(12.))
                    .overflow_hidden()
                    .relative()
                    .bg(vertical_linear_gradient(rgb(0x7A838C), rgb(0xF3FBFE)))
                    .child(
                        div()
                            .top_px()
                            .left(px(3.))
                            .absolute()
                            .overflow_hidden()
                            .w(px(6.))
                            .h(px(3.))
                            .rounded_t_full()
                            .bg(vertical_linear_gradient(rgb(0xFFFFFF), rgb(0x9EA3A9))),
                    ),
            )
    }

    fn render_traffic_lights(&self, width: impl Into<Length>) -> impl IntoElement {
        h_stack()
            .id("traffic-lights")
            .group("traffic-lights")
            .gap(px(6.))
            .w(width.into())
            .justify_center()
            .border_color(gpui::white().opacity(0.1))
            .child(self.render_traffic_light())
            .child(self.render_traffic_light())
            .child(self.render_traffic_light())
    }

    fn render_playback_button(
        &self,
        size: impl Into<Pixels>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let size = size.into();

        div()
            .id("some-playback-button")
            .relative()
            .flex_none()
            .w(size)
            .h(size + px(1.))
            .child(
                circle(size)
                    .absolute()
                    .bottom(px(0.))
                    .left(px(0.))
                    .bg(vertical_linear_gradient(rgb(0x5E5E5E), rgb(0xD5D3D6))),
            )
            .child(
                circle(size)
                    .border_1()
                    .border_color(rgb(0x737373))
                    .bg(rgb(0xF0F0F0)),
            )
            .active(|this| this.opacity(0.8))
            .on_click(cx.listener(move |this, event, cx| {
                println!("{:?}", event);
                cx.notify();
            }))
    }

    fn render_playback_buttons(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_stack()
            .gap(px(4.))
            .items_center()
            .child(self.render_playback_button(px(30.), cx))
            .child(self.render_playback_button(px(36.), cx))
            .child(self.render_playback_button(px(30.), cx))
    }

    fn render_volume_controls(&self) -> impl IntoElement {
        let current_volume: f32 = 0.7;
        let width: f32 = 75.0;
        let thumb_width: f32 = 12.0;
        let thumb_position = current_volume * width - (thumb_width / 2.0);

        h_stack()
            .ml(px(10.))
            .gap_1()
            .child(div().size(px(12.)).bg(gpui::red()))
            .child(
                h_stack()
                    .relative()
                    .child(
                        div()
                            .w(px(75.))
                            .h(px(5.))
                            .rounded_full()
                            .border_1()
                            .border_color(rgb(0x444444))
                            .bg(vertical_linear_gradient(rgb(0x666666), rgb(0x838383))),
                    )
                    .child(
                        circle(px(thumb_width))
                            .flex()
                            .items_center()
                            .justify_center()
                            .absolute()
                            .left(px(thumb_position))
                            .bg(rgb(0xFEFEFE))
                            .border_1()
                            .border_color(rgb(0x7C7C7C))
                            .child(
                                circle(px(4.0))
                                    .bg(vertical_linear_gradient(rgb(0x3D3D3D), rgb(0x9A9A9A))),
                            ),
                    ),
            )
            .child(div().size(px(12.)).bg(gpui::red()))
    }

    fn render_now_playing(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let current_track = self.state.read(cx).current_track.clone();
        let title = current_track.title().to_string();
        let artist = current_track.artist().to_string();

        let width: f32 = 350.;
        let height: f32 = 46.;

        let inner_element = v_stack()
            .flex_grow()
            .w_full()
            .gap(px(2.))
            .child(div().text_size(px(11.)).child(title))
            .child(div().text_size(px(11.)).child(artist))
            .child(
                h_stack()
                    .gap(px(4.))
                    .flex_grow()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(10.))
                            .child(current_track.current_time.format()),
                    )
                    .child(
                        div()
                            .flex_grow()
                            .items_center()
                            .h(px(9.))
                            .relative()
                            .border_1()
                            .border_color(rgb(0x000000))
                            .child(
                                circle(px(8.))
                                    .absolute()
                                    .top(px(-2.))
                                    .left(relative(current_track.percent_complete()))
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0x999999)),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(10.))
                            .child(current_track.time_remaining().format()),
                    ),
            );

        h_stack()
            .rounded(px(5.0))
            .bg(vertical_linear_gradient(rgb(0x56574F), rgb(0xE1E1E1)))
            .px_px()
            .flex_grow()
            .h(px(height))
            .w(px(width))
            .child(
                h_stack()
                    .w(px(width - 2.))
                    .h(px(height - 2.))
                    .px_px()
                    .flex_grow()
                    .rounded(px(4.0))
                    .bg(vertical_linear_gradient(rgb(0x969988), rgb(0xC1C4AF)))
                    .child(
                        h_stack()
                            .flex_grow()
                            .w(px(width - 4.))
                            .h(px(height - 4.))
                            .rounded(px(3.0))
                            .bg(rgb(0xD6DABF))
                            .gap(px(8.))
                            .child(div().size(px(11.)).bg(gpui::red()))
                            .child(inner_element)
                            .child(div().size(px(11.)).bg(gpui::red())),
                    ),
            )
    }

    fn render_search(&self) -> impl IntoElement {
        div()
    }

    fn render_browse(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .child(
                div()
                    .size(px(33.))
                    .rounded_full()
                    .bg(rgb(0xF0F0F0))
                    .border_1()
                    .border_color(rgb(0x5E5E5E)),
            )
            .child(div().mt(px(3.)).text_size(px(11.)).child("Browse"))
    }
}

struct GpuiTunes {
    state: Model<AppState>,
    focus_handle: FocusHandle,
}

impl Render for GpuiTunes {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let title_bar = cx.new_view(|_| TitleBar {
            state: self.state.clone(),
        });

        let library = cx.new_view(|_| LibraryContent {
            state: self.state.clone(),
        });

        let footer = cx.new_view(|_| Footer {
            state: self.state.clone(),
        });

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
            .child(title_bar.clone())
            .child(library.clone())
            .child(footer.clone())
            .on_click(|e, _cx| {
                println!("{:?}", e);
            })
        // .child(
        //     div()
        //         .occlude()
        //         .absolute()
        //         .size_full()
        //         .rounded(window_rounding)
        //         .border_1()
        //         .border_color(gpui::white().opacity(0.1)),
        // )
    }
}

impl FocusableView for GpuiTunes {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |cx| {
                let state = cx.new_model(|cx| AppState::new(cx));
                let focus_handle = cx.focus_handle();

                cx.new_view(|_cx| GpuiTunes {
                    state,
                    focus_handle,
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
