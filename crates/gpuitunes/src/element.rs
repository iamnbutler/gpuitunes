use gpui::prelude::FluentBuilder as _;
use gpui::*;

use crate::assets::Icon;

pub fn h_stack() -> Div {
    div().flex().items_center()
}

pub fn v_stack() -> Div {
    div().flex().flex_col()
}

#[derive(IntoElement)]
pub struct Spacer {
    width: Option<Length>,
    height: Option<Length>,
    grow: bool,
}

impl Spacer {
    pub fn new() -> Self {
        Spacer {
            width: None,
            height: None,
            grow: false,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn grow(mut self) -> Self {
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

pub fn spacer() -> Spacer {
    Spacer::new()
}

pub fn circle(size: impl Into<DefiniteLength>) -> Div {
    div().size(size.into()).flex_none().rounded_full()
}

pub fn vertical_linear_gradient(start: impl Into<Hsla>, stop: impl Into<Hsla>) -> Background {
    let start = linear_color_stop(start, 0.0);
    let end = linear_color_stop(stop, 1.0);

    gpui::linear_gradient(180.0, start, end)
}

pub fn large_icon(icon: Icon) -> Svg {
    svg()
        .size(px(16.))
        .flex_none()
        .path(icon.path())
        .text_color(rgb(0x000000))
}

pub fn small_icon(icon: Icon) -> Svg {
    svg()
        .size(px(14.))
        .flex_none()
        .path(icon.path())
        .text_color(rgb(0x000000))
}
