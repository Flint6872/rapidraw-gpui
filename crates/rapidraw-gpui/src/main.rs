mod actions;
mod components;
mod state;

use crate::state::AppState;
use actions::{OpenImage, ResetAdjustments};
use components::canvas::render_canvas;
use components::menus::setup_menus;
use components::sidebar::render_sidebar;
use gpui_kit::component::Root;
use gpui_kit::component::slider::{SliderEvent, SliderState};
use gpui_kit::*;
use gpui_util::ResultExt; // <--- ADDED: For .log_err()
use std::sync::Arc;

struct MainView {
    state: AppState,
    exposure_slider: Entity<SliderState>,
    contrast_slider: Entity<SliderState>,
    is_processing: bool,
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let exposure_slider = cx.new(|_| {
            SliderState::new()
                .min(-4.0)
                .max(4.0)
                .default_value(0.0)
                .step(0.01)
        });

        let contrast_slider = cx.new(|_| {
            SliderState::new()
                .min(0.2)
                .max(3.0)
                .default_value(1.0)
                .step(0.01)
        });

        cx.subscribe(&exposure_slider, |this, _, event: &SliderEvent, cx| {
            if let SliderEvent::Change(value) = event {
                this.state.adjustments.exposure = value.start();
                this.trigger_async_reprocess(cx);
            }
        })
        .detach();

        cx.subscribe(&contrast_slider, |this, _, event: &SliderEvent, cx| {
            if let SliderEvent::Change(value) = event {
                this.state.adjustments.contrast = value.start();
                this.trigger_async_reprocess(cx);
            }
        })
        .detach();

        Self {
            state: AppState::new(),
            exposure_slider,
            contrast_slider,
            is_processing: false,
        }
    }

    /// Spawns image processing onto a background thread and then updates the UI on the main thread.
    fn trigger_async_reprocess(&mut self, cx: &mut Context<Self>) {
        if self.is_processing {
            return;
        }
        self.is_processing = true;

        let base_buffer = self.state.base_buffer.clone();
        let adjustments = self.state.adjustments.clone();

        // Spawn heavy pixel processing onto a background thread
        let processing_task = cx.background_executor().spawn(async move {
            let pipeline = rapidraw_pipeline::RenderPipeline::new();
            pipeline.process(&base_buffer, &adjustments)
        });

        // Use the standard pattern for async updates with AsyncApp:
        // 1. Outer closure takes &mut AsyncApp
        // 2. Clone AsyncApp into an owned instance
        // 3. Move the owned instance into the async block
        // 4. Pass &mut of the cloned instance to .update()
        cx.spawn(|this: WeakEntity<Self>, async_cx_ref: &mut AsyncApp| {
            let mut async_cx = async_cx_ref.clone(); // <--- CRITICAL: Clone AsyncApp
            async move {
                let processed = processing_task.await;

                this.update(&mut async_cx, |this, cx| {
                    // <--- Pass &mut of the cloned AsyncApp
                    let rgba_img = image::RgbaImage::from_raw(
                        processed.width,
                        processed.height,
                        (*processed.rgba_data).clone(),
                    )
                    .unwrap_or_else(|| image::RgbaImage::new(1, 1));

                    let frame = image::Frame::new(rgba_img);
                    let new_render_image = Arc::new(RenderImage::new(vec![frame]));

                    let old_render_image =
                        std::mem::replace(&mut this.state.render_image, new_render_image);
                    cx.drop_image(old_render_image, None);

                    this.is_processing = false;
                    cx.notify();
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn open_file_dialog(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "tiff"])
            .pick_file()
        {
            //let path_clone = path.clone();

            cx.spawn(move |this: WeakEntity<Self>, async_cx_ref: &mut AsyncApp| {
                // 1. Clone the &mut AsyncApp into an owned instance right at the start
                let mut async_cx = async_cx_ref.clone();
                let path_clone = path.clone();

                async move {
                    let load_result = async_cx
                        .background_executor()
                        .spawn({
                            let path = path_clone.clone();
                            async move { rapidraw_core::load_image_file(path) }
                        })
                        .await;

                    // 2. Pass &mut async_cx into update
                    this.update(&mut async_cx, |this, cx| {
                        match load_result {
                            Ok(buffer) => {
                                this.state.base_buffer = buffer;
                                this.state.current_file = Some(path_clone);
                                this.trigger_async_reprocess(cx);
                            }
                            Err(err) => eprintln!("Failed to load image: {err}"),
                        }
                        cx.notify();
                    })
                    .ok();
                }
            })
            .detach();
        }
    }

    pub fn reset_adjustments(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.state.reset_adjustments();
        self.exposure_slider.update(cx, |s, cx| {
            s.set_value(0.0, window, cx);
        });
        self.contrast_slider.update(cx, |s, cx| {
            s.set_value(1.0, window, cx);
        });
        self.trigger_async_reprocess(cx);
        cx.notify();
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let file_name = self
            .state
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str());

        div()
            .id("main-view-container")
            .flex()
            .size_full()
            .on_action(cx.listener(|this, _: &OpenImage, window, cx| {
                this.open_file_dialog(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ResetAdjustments, window, cx| {
                this.reset_adjustments(window, cx);
            }))
            .child(render_sidebar(
                self.state.adjustments.exposure,
                self.state.adjustments.contrast,
                &self.exposure_slider,
                &self.contrast_slider,
                cx,
                |this, _event, window, cx| this.open_file_dialog(window, cx),
                |this, _event, window, cx| this.reset_adjustments(window, cx),
            ))
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .child(render_canvas(self.state.render_image.clone(), file_name)),
            )
    }
}

fn main() {
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
        gpui_kit::component::init(cx);
        setup_menus(cx);

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
            let view = cx.new(|cx| MainView::new(cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();

        cx.activate(true);
    });
}
