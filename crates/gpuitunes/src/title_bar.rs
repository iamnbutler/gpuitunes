use crate::{assets::Icon, AppState};
use crate::{element::*, FullScreen, Minimize, Quit};
use gpui::*;
use smallvec::smallvec;

// TODO: Move to playback
actions!(
    playback,
    [
        SkipPrev,
        SkipNext,
        TogglePlayback,
        Play,
        Pause,
        Restart,
        VolumeIncrease,
        VolumeDecrease
    ]
);

#[derive(Clone, Copy, Debug)]
enum WindowButtonType {
    Close,
    Minimize,
    FullScreen,
}

impl WindowButtonType {
    fn bg(&self) -> Background {
        match self {
            WindowButtonType::Close => vertical_linear_gradient(rgb(0xC45554), rgb(0xFEB2A4)),
            WindowButtonType::Minimize => vertical_linear_gradient(rgb(0xEDB353), rgb(0xFEEA74)),
            WindowButtonType::FullScreen => vertical_linear_gradient(rgb(0x83A942), rgb(0xD4F596)),
        }
    }
    fn id(&self) -> ElementId {
        match self {
            WindowButtonType::Close => ElementId::Name("close".into()),
            WindowButtonType::Minimize => ElementId::Name("minimize".into()),
            WindowButtonType::FullScreen => ElementId::Name("fullscreen".into()),
        }
    }
}

#[derive(IntoElement)]
struct TrafficLight {
    button_type: WindowButtonType,
}

impl TrafficLight {
    fn new(button_type: WindowButtonType, _cx: &mut WindowContext) -> Self {
        TrafficLight { button_type }
    }

    fn close(cx: &mut WindowContext) -> Self {
        TrafficLight::new(WindowButtonType::Close, cx)
    }

    fn minimize(cx: &mut WindowContext) -> Self {
        TrafficLight::new(WindowButtonType::Minimize, cx)
    }

    fn fullscreen(cx: &mut WindowContext) -> Self {
        TrafficLight::new(WindowButtonType::FullScreen, cx)
    }
}

impl RenderOnce for TrafficLight {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let button_type = self.button_type;

        circle(px(14.))
            .id(button_type.id())
            .rounded_full()
            .overflow_hidden()
            .p_px()
            .bg(vertical_linear_gradient(rgb(0x101010), rgb(0x95999C)))
            .shadow(highlight_ring_shadow())
            .on_click(move |_, cx| match button_type {
                WindowButtonType::Close => cx.dispatch_action(Box::new(Quit)),
                WindowButtonType::Minimize => cx.dispatch_action(Box::new(Minimize)),
                WindowButtonType::FullScreen => cx.dispatch_action(Box::new(FullScreen)),
            })
            .child(
                circle(px(12.))
                    .overflow_hidden()
                    .relative()
                    .bg(vertical_linear_gradient(rgb(0x7A838C), rgb(0xF3FBFE)))
                    .group_hover("title-bar", |this| this.bg(button_type.bg()))
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
}

pub struct TitleBar {
    state: Model<AppState>,
}

impl TitleBar {
    pub fn new(state: Model<AppState>, _cx: &mut ViewContext<Self>) -> Self {
        // cx.subscribe(
        //     &state,
        //     |_this, _model, _event: &CurrentTimeChangedEvent, cx| {
        //         cx.notify();
        //     },
        // )
        // .detach();

        TitleBar {
            state: state.clone(),
        }
    }
}

impl TitleBar {
    fn render_traffic_lights(&self, cx: &mut WindowContext) -> impl IntoElement {
        h_stack()
            .id("traffic-lights")
            .group("traffic-lights")
            .absolute()
            .top(px(5.))
            .left(px(8.))
            .gap(px(7.))
            .justify_center()
            .border_color(gpui::white().opacity(0.1))
            .child(TrafficLight::close(cx))
            .child(TrafficLight::minimize(cx))
            .child(TrafficLight::fullscreen(cx))
    }

    fn render_playback_button(
        &self,
        size: impl Into<Pixels>,
        icon: Icon,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let size = size.into();

        div()
            .id("some-playback-button")
            .relative()
            .flex_none()
            .w(size)
            .h(size)
            .rounded_full()
            .shadow(highlight_ring_shadow())
            .child(
                circle(size)
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .border_1()
                    .border_color(rgb(0x737373))
                    .bg(rgb(0xF0F0F0))
                    .child(large_icon(icon).relative().left(match icon {
                        Icon::Next => px(1.),
                        Icon::Previous => px(-1.),
                        _ => px(0.),
                    })),
            )
            .active(|this| this.opacity(0.8))
            .on_click(cx.listener(move |_, event, cx| {
                println!("{:?}", event);
                cx.notify();
            }))
    }

    fn render_playback_buttons(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_stack()
            .top(px(5.))
            .gap(px(4.))
            .items_center()
            .child(self.render_playback_button(px(31.), Icon::Previous, cx))
            .child(self.render_playback_button(px(37.), Icon::Pause, cx))
            .child(self.render_playback_button(px(31.), Icon::Next, cx))
    }

    fn render_volume_controls(&self) -> impl IntoElement {
        let current_volume: f32 = 0.7;
        let width: f32 = 75.0;
        let thumb_width: f32 = 12.0;
        let thumb_position = current_volume * width - (thumb_width / 2.0);

        h_stack()
            .ml(px(10.))
            .gap_1()
            .child(small_icon(Icon::VolumeLow))
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
            .child(small_icon(Icon::VolumeHigh))
    }

    fn render_now_playing(&self, _cx: &ViewContext<Self>) -> impl IntoElement {
        // let current_track = self.state.read(cx).current_track();

        // let width: f32 = 350.;
        // let height: f32 = 46.;

        // let inner_element = match current_track {
        //     Some(track) => {
        //         let title = track.title().to_string();
        //         let artist = track.artist().to_string();

        //         v_stack()
        //             .flex_grow()
        //             .w_full()
        //             .child(
        //                 h_stack()
        //                     .pt(px(4.))
        //                     .flex_shrink_0()
        //                     .w_full()
        //                     .justify_center()
        //                     .child(div().flex_none().text_size(px(11.)).child(title)),
        //             )
        //             .child(
        //                 h_stack()
        //                     .flex_shrink_0()
        //                     .w_full()
        //                     .justify_center()
        //                     .child(div().flex_none().text_size(px(11.)).child(artist)),
        //             )
        //             .child(
        //                 h_stack()
        //                     .h(px(11.))
        //                     .pb(px(2.))
        //                     .gap(px(4.))
        //                     .flex_grow()
        //                     .items_center()
        //                     .child(
        //                         h_stack()
        //                             .flex_none()
        //                             .text_size(px(10.))
        //                             .child(track.current_time().format()),
        //                     )
        //                     .child(
        //                         div()
        //                             .mb_px()
        //                             .flex_grow()
        //                             .items_center()
        //                             .h(px(9.))
        //                             .relative()
        //                             .border_1()
        //                             .border_color(rgb(0x000000))
        //                             .child(
        //                                 circle(px(5.))
        //                                     .absolute()
        //                                     .top(px(1.))
        //                                     .left(relative(track.progress()))
        //                                     .bg(rgb(0x000000)),
        //                             ),
        //                     )
        //                     .child(
        //                         h_stack()
        //                             .flex_none()
        //                             .text_size(px(10.))
        //                             .child(track.time_remaining().format()),
        //                     ),
        //             )
        //     }
        //     None => v_stack()
        //         .flex_grow()
        //         .w_full()
        //         .justify_center()
        //         .child(div().text_size(px(11.)).child("No track playing")),
        // };

        // h_stack()
        //     .rounded(px(5.0))
        //     .bg(vertical_linear_gradient(rgb(0x56574F), rgb(0xE1E1E1)))
        //     .px_px()
        //     .flex_grow()
        //     .h(px(height))
        //     .w(px(width))
        //     .child(
        //         h_stack()
        //             .w(px(width - 2.))
        //             .h(px(height - 2.))
        //             .px_px()
        //             .flex_grow()
        //             .rounded(px(4.0))
        //             .bg(vertical_linear_gradient(rgb(0x969988), rgb(0xC1C4AF)))
        //             .child(
        //                 h_stack()
        //                     .flex_grow()
        //                     .w(px(width - 4.))
        //                     .h(px(height - 4.))
        //                     .rounded(px(3.0))
        //                     .bg(rgb(0xD6DABF))
        //                     .gap(px(8.))
        //                     .child(div().size(px(11.)).bg(gpui::red()))
        //                     .child(inner_element)
        //                     .child(div().size(px(11.)).bg(gpui::red())),
        //             ),
        //     )
        div()
    }

    fn render_search(&self) -> impl IntoElement {
        let input_width: f32 = 134.;
        let input_height: f32 = 20.;

        h_stack()
            .mr(px(20.))
            .flex_none()
            .rounded_full()
            .w(px(input_width))
            .h(px(input_height))
            .bg(vertical_linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
            .child(
                h_stack()
                    .flex_none()
                    .rounded_full()
                    .gap(px(4.))
                    .px(px(3.))
                    .w(px(input_width - 2.))
                    .h(px(input_height - 2.))
                    .bg(rgb(0xFFFFFF))
                    .child(small_icon(Icon::MagnifyingGlass))
                    .child(
                        h_stack()
                            .flex_1()
                            .text_size(px(11.))
                            .line_height(px(11.))
                            .child("Search..."),
                    )
                    .child(small_icon(Icon::XCircle).text_color(rgb(0xB3B3B3))),
            )
    }

    fn render_browse(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .pr(px(24.))
            .child(
                h_stack()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .size(px(33.))
                    .rounded_full()
                    .bg(rgb(0xF0F0F0))
                    .border_1()
                    .border_color(rgb(0x5E5E5E))
                    .child(
                        h_stack()
                            .flex_none()
                            .items_center()
                            .size(px(24.))
                            .child(
                                h_stack()
                                    .items_center()
                                    .justify_center()
                                    .absolute()
                                    .top(px(6.))
                                    .left(px(6.))
                                    .size(px(12.))
                                    .rounded_full()
                                    .overflow_hidden()
                                    .child(
                                        div()
                                            .size(px(5.))
                                            .rounded_full()
                                            .overflow_hidden()
                                            // .bg(rgb(0x000000))
                                            .shadow(smallvec![
                                                BoxShadow {
                                                    color: hsla(120.0 / 360., 1.0, 0.55, 0.9), // Green
                                                    offset: point(px(0.), px(-4.)),
                                                    blur_radius: px(2.),
                                                    spread_radius: px(1.),
                                                },
                                                BoxShadow {
                                                    color: hsla(60.0 / 360., 1.0, 0.55, 0.9), // Yellow
                                                    offset: point(px(4.), px(0.)),
                                                    blur_radius: px(2.),
                                                    spread_radius: px(1.),
                                                },
                                                BoxShadow {
                                                    color: hsla(330.0 / 360., 1.0, 0.55, 0.9), // Pink
                                                    offset: point(px(0.), px(4.)),
                                                    blur_radius: px(2.),
                                                    spread_radius: px(1.),
                                                },
                                                BoxShadow {
                                                    color: hsla(240.0 / 360., 1.0, 0.55, 0.9), // Blue
                                                    offset: point(px(-4.), px(0.)),
                                                    blur_radius: px(2.),
                                                    spread_radius: px(1.),
                                                },
                                            ]),
                                    ),
                            )
                            .child(
                                svg()
                                    .absolute()
                                    .text_color(rgb(0x414141))
                                    .size(px(24.))
                                    .path(Icon::Eye.as_static_str()),
                            ),
                    ),
            )
            .child(div().mt(px(3.)).text_size(px(11.)).child("Browse"))
    }
}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_stack()
            .group("title-bar")
            .relative()
            .w_full()
            .bg(vertical_linear_gradient(rgb(0xC5C5C5), rgb(0x969696)))
            .border_b_1()
            .border_color(rgb(0x414141))
            // TODO: Should be able to drag the app from the whole title bar
            .child(self.render_traffic_lights(cx))
            .child(
                h_stack()
                    .id("title-bar")
                    .h(px(21.))
                    .relative()
                    .w_full()
                    .flex_none()
                    .child(div().flex_1())
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .top(px(1.))
                            .left(px(-21.))
                            .text_size(px(13.))
                            .font_weight(FontWeight::MEDIUM)
                            .child(div().text_color(rgb(0x888888)).child("gpu"))
                            .child(div().child("iTunes")),
                    )
                    .child(div().flex_1())
                    .justify_between(),
            )
            .child(
                div()
                    .flex()
                    .items_start()
                    .h(px(54.))
                    .child(
                        h_stack()
                            .relative()
                            .flex_none()
                            .justify_start()
                            .child(spacer().width(px(28.)))
                            .child(self.render_playback_buttons(cx))
                            .child(self.render_volume_controls()),
                    )
                    .child(
                        h_stack()
                            .flex_1()
                            .flex_shrink_0()
                            .w_full()
                            .justify_center()
                            .child(self.render_now_playing(cx)),
                    )
                    .child(
                        h_stack()
                            .flex_none()
                            .h_full()
                            .justify_end()
                            // .child(div().flex_1().child(""))
                            .child(
                                v_stack()
                                    .h(px(46.))
                                    .child(h_stack().h(px(32.)).child(self.render_search())),
                            )
                            .child(
                                h_stack()
                                    .h(px(46.))
                                    .w(px(38.))
                                    .justify_center()
                                    .flex_none()
                                    .child(self.render_browse()),
                            ),
                    ),
            )
    }
}
