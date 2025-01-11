#![allow(unused, dead_code)]

use gpui::*;
use library::{Library, NowPlaying};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};

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
}
