#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use app::{AppState, AppWindow};
use assets::Assets;
use gpui::*;
use library::Library;

mod app;
mod assets;
mod element;
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

                let library = Library::default();

                let library = cx.new_model(|_| library);

                cx.new_view(|cx| AppWindow::new(library, state.clone(), cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
