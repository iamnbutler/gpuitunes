#![allow(unused, dead_code)]

use gpui::*;
use library::{Library, NowPlaying};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::title_bar::TitleBar;

const UPDATE_INTERVAL: Duration = Duration::from_millis(250);

pub struct AppState {
    pending_update: Option<Task<()>>,
}

pub struct UpdateTriggered;

impl AppState {
    pub fn new(cx: &mut App) -> Self {
        AppState {
            pending_update: None,
        }
    }

    fn init_update(&mut self, cx: &mut Context<Self>) {
        if self.pending_update.is_none() {
            self.pending_update = Some(self.start_updates(cx));
        }
    }

    fn start_updates(&self, cx: &mut Context<Self>) -> Task<()> {
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
    window: WeakEntity<AppWindow>,
    library: Entity<Library>,
    now_playing: Entity<NowPlaying>,
}

impl Sidebar {
    pub fn new(
        window: WeakEntity<AppWindow>,
        library: Entity<Library>,
        now_playing: Entity<NowPlaying>,
    ) -> Self {
        Sidebar {
            window,
            library,
            now_playing,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

struct LibraryView {
    window: WeakEntity<AppWindow>,
    library: Entity<Library>,
    now_playing: Entity<NowPlaying>,
    focus_handle: FocusHandle,
}

impl LibraryView {
    pub fn new(
        window: WeakEntity<AppWindow>,
        library: Entity<Library>,
        now_playing: Entity<NowPlaying>,
        cx: &mut Context<Self>,
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl Focusable for LibraryView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct StatusBar {
    window: WeakEntity<AppWindow>,
    library: Entity<Library>,
}

impl StatusBar {
    pub fn new(window: WeakEntity<AppWindow>, library: Entity<Library>) -> Self {
        StatusBar { window, library }
    }
}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

pub struct AppWindow {
    // sidebar: Entity<Sidebar>,
    // For now is just the library, but could
    // be a slot for any activated view
    // active_view: Entity<LibraryView>,
    // status_bar: Entity<StatusBar>,
    // library: Entity<Library>,
    // now_playing: Entity<NowPlaying>,
    focus_handle: FocusHandle,
    app_state: Arc<AppState>,
    _subscriptions: Vec<Subscription>,
    // _schedule_serialize: Option<Task<()>>,
}

impl AppWindow {
    pub fn new(
        library: Entity<Library>,
        app_state: Arc<AppState>,
        window: &mut Window,
        cx: &mut Context<Self>,
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
        // cx.on_focus_lost(|this, cx| {
        //     let focus_handle = this.focus_handle(cx);
        //     cx.focus(&focus_handle);
        // })
        // .detach();

        // let weak_handle = cx.entity().downgrade();

        let app_state = Arc::new(AppState::new(cx));

        // let now_playing = cx.new(|_| NowPlaying::default());

        // let sidebar =
        //     cx.new(|cx| Sidebar::new(weak_handle.clone(), library.clone(), now_playing.clone()));
        // let library_view = cx.new(|cx| {
        //     LibraryView::new(
        //         weak_handle.clone(),
        //         library.clone(),
        //         now_playing.clone(),
        //         cx,
        //     )
        // });
        // let status_bar = cx.new(|cx| StatusBar::new(weak_handle.clone(), library.clone()));

        AppWindow {
            // sidebar,
            // active_view: library_view,
            // status_bar,
            // library,
            // now_playing,
            app_state,
            focus_handle: cx.focus_handle(),
            _subscriptions: Vec::new(),
        }
    }
}

impl AppWindow {
    pub fn app_state(&self) -> &Arc<AppState> {
        &self.app_state
    }

    // pub fn library(&self) -> &Entity<Library> {
    //     &self.library
    // }
}

impl Focusable for AppWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AppWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // This should be more like 4.0, but later macOS versions have
        // a higher default window border radius
        let window_rounding = px(10.0);
        let state = cx.new(|cx| AppState::new(cx));

        let title_bar = cx.new(|cx| TitleBar::new(state.clone(), window, cx));

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
            .child(title_bar)
    }
}

impl EventEmitter<Event> for AppWindow {}

pub enum Event {
    PlaybackStarted,
    PlaybackPaused,
    PlaybackStopped,
}
