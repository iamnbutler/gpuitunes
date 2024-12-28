#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use app::{AppState, AppWindow, Library};
use assets::Assets;
use gpui::*;

mod app;
mod assets;
mod element;
mod library;
mod title_bar;

fn main() {
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |cx| {
                let state = Arc::new(AppState::new(cx));

                let library = Library {
                    tracks: HashMap::new(),
                    track_order: Vec::new(),
                    columns: Vec::new(),
                };

                let library = cx.new_model(|_| library);

                cx.new_view(|cx| AppWindow::new(library, state.clone(), cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
