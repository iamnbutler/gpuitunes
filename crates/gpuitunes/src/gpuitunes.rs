#![allow(dead_code)]

use std::sync::Arc;

use crate::title_bar::TitleBar;
use crate::{assets::Icon, library::CurrentTrack};
use assets::Assets;
use gpui::*;
use library::{Column, ColumnKind, Library, SerializableTrack, Track, TrackId};
use prelude::FluentBuilder as _;

mod assets;
mod library;
mod title_bar;

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

fn large_icon(icon: Icon) -> Svg {
    svg()
        .size(px(16.))
        .flex_none()
        .path(icon.path())
        .text_color(rgb(0x000000))
}

fn small_icon(icon: Icon) -> Svg {
    svg()
        .size(px(14.))
        .flex_none()
        .path(icon.path())
        .text_color(rgb(0x000000))
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
                Library::new()
            }
        };

        let mut state = AppState {
            current_track: CurrentTrack::new(default_track),
            library: Arc::new(library),
            sidebar_width: None,
        };

        state.current_track.current_time = 37.into();

        state
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let sidebar_width = state.sidebar_width.unwrap_or(182.);

        v_stack()
            .h_full()
            .flex_none()
            .w(px(sidebar_width))
            .border_r_1()
            .border_color(rgb(0x3F3F3F))
            .child(
                v_stack()
                    .size_full()
                    .child(
                        v_stack()
                            .bg(rgb(0xE8ECF7))
                            .flex_1()
                            .child(
                                div()
                                    .h(px(16.))
                                    .border_b_1()
                                    .border_color(rgb(0x666664))
                                    .bg(vertical_linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
                                    .child("Source"),
                            )
                            .child(div()),
                    )
                    .child(
                        v_stack()
                            .flex_1()
                            .bg(rgb(0xFFFFFF))
                            .border_t_1()
                            .border_color(rgb(0x3F413C))
                            .child(
                                div()
                                    .h(px(16.))
                                    .border_b_1()
                                    .border_color(rgb(0x666664))
                                    .bg(vertical_linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
                                    .child("Now Playing"),
                            )
                            .child(div()),
                    ),
            )
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
                            .on_click(cx.listener(move |_, event, cx| {
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

struct GpuiTunes {
    state: Model<AppState>,
    focus_handle: FocusHandle,
}

impl Render for GpuiTunes {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let title_bar = cx.new_view(|_| TitleBar::new(self.state.clone()));
        let sidebar = cx.new_view(|_| Sidebar {
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
            .child(
                h_stack()
                    .w_full()
                    .flex_1()
                    .overflow_hidden()
                    .child(sidebar.clone())
                    .child(library.clone()),
            )
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
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
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
