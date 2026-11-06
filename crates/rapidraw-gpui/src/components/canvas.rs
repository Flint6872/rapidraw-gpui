use gpui_kit::*;
use std::sync::Arc;

pub fn render_canvas(render_image: Arc<RenderImage>, file_name: Option<&str>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(rgb(0x121214))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .px_4()
                .py_2()
                .bg(rgb(0x1e1e22))
                .border_b_1()
                .border_color(rgb(0x2d2d32))
                .child(div().text_sm().font_weight(FontWeight::BOLD).child(format!(
                    "RapidRAW — {}",
                    file_name.unwrap_or("Test Pattern (No file loaded)")
                ))),
        )
        .child(
            div()
                .flex_1()
                .size_full()
                .p_4()
                .justify_center()
                .items_center()
                .child(
                    img(ImageSource::Render(render_image))
                        .size_full()
                        .object_fit(ObjectFit::Contain),
                ),
        )
}
