use gpui_kit::component::Root;
use gpui_kit::*;

struct MainView;

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x121214))
            .text_color(rgb(0xe4e4e7))
            .justify_center()
            .items_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child("RapidRAW"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xa1a1aa))
                            .child("GPUI Canvas Shell Ready"),
                    ),
            )
    }
}

fn main() {
    // Linux Wayland workaround matching reference setup
    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() && std::env::var("RAPIDRAW_X11_FORCED").is_err()
        {
            if let Ok(current_exe) = std::env::current_exe() {
                let status = std::process::Command::new(current_exe)
                    .args(std::env::args().skip(1))
                    .env_remove("WAYLAND_DISPLAY")
                    .env("RAPIDRAW_X11_FORCED", "1")
                    .status();

                if let Ok(code) = status {
                    std::process::exit(code.code().unwrap_or(0));
                }
            }
        }
    }

    gpui_kit::application().run(|cx: &mut App| {
        // Initialize gpui-kit internal state & components
        gpui_kit::component::init(cx);

        let initial_bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(initial_bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("RapidRAW".into()),
                appears_transparent: cfg!(target_os = "macos"),
                traffic_light_position: if cfg!(target_os = "macos") {
                    Some(point(px(16.0), px(16.0)))
                } else {
                    None
                },
                ..Default::default()
            }),
            window_decorations: if cfg!(target_os = "linux") {
                Some(WindowDecorations::Server)
            } else {
                None
            },
            focus: true,
            show: true,
            kind: WindowKind::Normal,
            is_resizable: true,
            is_movable: true,
            ..Default::default()
        };

        cx.open_window(window_options, |window: &mut Window, cx: &mut App| {
            let view = cx.new(|_cx| MainView);
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();

        cx.activate(true);
    });
}
