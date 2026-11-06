use gpui_kit::component::slider::{Slider, SliderState};
use gpui_kit::*;

pub fn render_sidebar<V: 'static>(
    exposure: f32,
    contrast: f32,
    exposure_slider: &Entity<SliderState>,
    contrast_slider: &Entity<SliderState>,
    cx: &mut Context<V>,
    on_open: impl Fn(&mut V, &ClickEvent, &mut Window, &mut Context<V>) + 'static,
    on_reset: impl Fn(&mut V, &ClickEvent, &mut Window, &mut Context<V>) + 'static,
) -> impl IntoElement {
    div()
        .w_72()
        .h_full()
        .bg(rgb(0x18181b))
        .border_r_1()
        .border_color(rgb(0x27272a))
        .p_4()
        .flex()
        .flex_col()
        .gap_6()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xa1a1aa))
                        .child("FILE OPERATIONS"),
                )
                .child(
                    div()
                        .id("open-image-btn")
                        .px_3()
                        .py_2()
                        .bg(rgb(0x27272a))
                        .hover(|s| s.bg(rgb(0x3f3f46)))
                        .rounded_md()
                        .cursor_pointer()
                        .text_xs()
                        .text_color(rgb(0xf4f4f5))
                        .child("Open Image File...")
                        .on_click(cx.listener(move |view, event, window, cx| {
                            on_open(view, event, window, cx);
                        })),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xa1a1aa))
                        .child("NON-DESTRUCTIVE MATH"),
                )
                // Exposure Control
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .text_xs()
                                .text_color(rgb(0xe4e4e7))
                                .child("Exposure (EV)")
                                .child(
                                    div()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x38bdf8))
                                        .child(format!("{:.2}", exposure)),
                                ),
                        )
                        .child(Slider::new(exposure_slider)),
                )
                // Contrast Control
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .text_xs()
                                .text_color(rgb(0xe4e4e7))
                                .child("Contrast")
                                .child(
                                    div()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x38bdf8))
                                        .child(format!("{:.2}x", contrast)),
                                ),
                        )
                        .child(Slider::new(contrast_slider)),
                ),
        )
        .child(
            div()
                .id("reset-adjustments-btn")
                .mt_auto()
                .px_3()
                .py_2()
                .bg(rgb(0x3f3f46))
                .hover(|s| s.bg(rgb(0x52525b)))
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(rgb(0xf4f4f5))
                .child("Reset Adjustments")
                .on_click(cx.listener(move |view, event, window, cx| {
                    on_reset(view, event, window, cx);
                })),
        )
}
