pub mod actions;
pub mod components;
pub mod state;

use actions::{OpenImage, ResetAdjustments};
use components::canvas::render_canvas;
use components::menus::setup_menus;
use components::sidebar::render_sidebar;
use gpui_kit::component::Root;
use gpui_kit::component::slider::{SliderEvent, SliderState};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_util::ResultExt;
use state::AppState;
use std::sync::Arc;

pub fn is_mobile() -> bool {
    cfg!(any(target_os = "ios", target_os = "android"))
}

pub struct MainView {
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

    pub fn trigger_async_reprocess(&mut self, cx: &mut Context<Self>) {
        if self.is_processing {
            return;
        }
        self.is_processing = true;

        let base_buffer = self.state.base_buffer.clone();
        let adjustments = self.state.adjustments.clone();

        let processing_task = cx.background_executor().spawn(async move {
            let pipeline = rapidraw_pipeline::RenderPipeline::new();
            pipeline.process(&base_buffer, &adjustments)
        });

        cx.spawn(|this: WeakEntity<Self>, async_cx_ref: &mut AsyncApp| {
            let mut async_cx = async_cx_ref.clone();
            async move {
                let processed = processing_task.await;

                this.update(&mut async_cx, |this, cx| {
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
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp", "tiff"])
                .pick_file()
            {
                cx.spawn(move |this: WeakEntity<Self>, async_cx_ref: &mut AsyncApp| {
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

        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            let _ = cx;
            eprintln!("Native mobile file picker will be integrated via platform channel.");
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

        let is_touch = is_mobile();

        // Responsive layout: Horizontal split on desktop, Vertical column on mobile
        let container = div()
            .id("main-view-container")
            .flex()
            .size_full()
            .when(is_touch, |d| d.flex_col_reverse())
            .on_action(cx.listener(|this, _: &OpenImage, window, cx| {
                this.open_file_dialog(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ResetAdjustments, window, cx| {
                this.reset_adjustments(window, cx);
            }));

        container
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

/// Core application bootstrap shared across Desktop, iOS, and Android
pub fn run_app() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kit::component::init(cx);

        // Menus are only applicable to desktop
        if !is_mobile() {
            setup_menus(cx);
        }

        let window_options = if is_mobile() {
            // Mobile: Fullscreen window without desktop borders
            WindowOptions {
                kind: WindowKind::Normal,
                window_decorations: None,
                ..Default::default()
            }
        } else {
            // Desktop: 1280x800 centered with native decorations
            let initial_bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
            WindowOptions {
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
            }
        };

        cx.open_window(window_options, |window: &mut Window, cx: &mut App| {
            let view = cx.new(|cx| MainView::new(cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();

        cx.activate(true);
    });
}

// Android native entrypoint
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(_app: android_activity::AndroidApp) {
    run_app();
}

// iOS native entrypoint
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn rapidraw_ios_main() {
    run_app();
}
}

// Android native entrypoint (called by the Android host)
#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: android_activity::AndroidApp) {
    gpui_mobile::AndroidApp::start(app, |cx: &mut App| {
        start_rapidraw_ui_common(cx);
    });
}

// iOS native entrypoint (called by the iOS host)
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn rapidraw_ios_main() {
    gpui_mobile::IosApp::start(|cx: &mut App| {
        start_rapidraw_ui_common(cx);
    });
}
