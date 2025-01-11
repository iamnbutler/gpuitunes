#![allow(dead_code)]

use std::sync::Arc;

use app::{AppState, AppWindow};
use assets::Assets;
use gpui::*;
use library::Library;

mod app;
mod assets;
mod element;
mod title_bar;

actions!(gpuitunes, [Quit]);

fn main() {
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "gpuiTunes".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

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
    });
}
