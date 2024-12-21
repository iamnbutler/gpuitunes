#![allow(dead_code)]

use gpui::*;
use prelude::FluentBuilder as _;
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

    fn grow(mut self, grow: bool) -> Self {
        self.grow = grow;
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

fn linear_gradient(start: impl Into<Hsla>, stop: impl Into<Hsla>) -> Background {
    let start = linear_color_stop(start, 0.0);
    let end = linear_color_stop(stop, 1.0);

    gpui::linear_gradient(180.0, start, end)
}

struct TitleBar {}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let traffic_lights_width = px(62.);

        v_stack()
            .debug_below()
            .h(px(64.))
            .w_full()
            .bg(linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
            .border_b_1()
            .border_color(rgb(0x414141))
            .child(
                h_stack()
                    .id("title-bar")
                    .h(px(20.))
                    .w_full()
                    .justify_between()
                    .child(self.render_traffic_lights(traffic_lights_width))
                    .child(div().child("gpuItunes"))
                    .child(spacer().width(traffic_lights_width)),
            )
    }
}

impl TitleBar {
    fn render_traffic_light(&self) -> impl IntoElement {
        circle(px(12.))
            .rounded_full()
            .overflow_hidden()
            .p_px()
            .bg(linear_gradient(rgb(0x101010), rgb(0x95999C)))
            .shadow(smallvec![BoxShadow {
                color: hsla(0.0, 1., 1., 0.36),
                offset: point(px(0.), px(1.)),
                blur_radius: px(1.),
                spread_radius: px(1.),
            }])
            .child(
                circle(px(10.))
                    .overflow_hidden()
                    .relative()
                    .bg(linear_gradient(rgb(0x7A838C), rgb(0xF3FBFE)))
                    .child(
                        div()
                            .top_px()
                            .left(px(3.))
                            .absolute()
                            .overflow_hidden()
                            .w(px(4.))
                            .h(px(3.))
                            .rounded_t_full()
                            .bg(linear_gradient(rgb(0xFFFFFF), rgb(0x9EA3A9))),
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

    fn render_playback_buttons(&self) -> impl IntoElement {
        div()
    }

    fn render_volume_controls(&self) -> impl IntoElement {
        div()
    }

    fn render_now_playing(&self) -> impl IntoElement {
        div()
    }

    fn render_search(&self) -> impl IntoElement {
        div()
    }

    fn render_browse(&self) -> impl IntoElement {
        div()
    }
}

struct GpuiTunes {
    text: SharedString,
}

impl Render for GpuiTunes {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let title_bar = cx.new_view(|_| TitleBar {});
        // This should be more like 4.0, but later macOS versions have
        // a higher default window border radius
        let window_rounding = px(10.0);

        div()
            .flex()
            .flex_col()
            .rounded(window_rounding)
            .overflow_hidden()
            .relative()
            .bg(rgb(0xFEFFFF))
            .size_full()
            .font_family("Helvetica")
            .line_height(px(14.))
            .text_color(rgb(0x0F1219))
            .text_size(px(14.))
            .child(title_bar.clone())
            .child(div().flex_1())
            .child(
                div()
                    .absolute()
                    .size_full()
                    .rounded(window_rounding)
                    .border_1()
                    .border_color(gpui::white().opacity(0.1)),
            )
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
                cx.new_view(|_cx| GpuiTunes {
                    text: "World".into(),
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
