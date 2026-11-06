# RapidRAW GPUI Migration Roadmap: Desktop-First to Cross-Platform Mobile

This roadmap outlines a strict, phase-by-phase migration strategy for RapidRAW. Every phase must compile and pass validation tests before proceeding to the next, ensuring architectural stability.

---

## Phase 1: GPUI Desktop App Bootstrap & Window Shell
**Goal:** Get the GPUI application up and running visually on the desktop immediately to validate the framework and windowing loop.

- [ ] **Step 1.1:** Initialize the root Cargo workspace (`rapidraw-ui`, `rapidraw-core`, `rapidraw-pipeline`).
- [ ] **Step 1.2:** Configure `rapidraw-ui` to boot a native desktop window using `gpui-kit`.
- [ ] **Step 1.3:** Implement a basic placeholder view with a custom layout container and clear event loop.
- [ **Verification:** ] Launch the desktop binary; verify an empty, responsive native window launches instantly with 0% idle CPU and minimal memory overhead.

---

## Phase 2: Core Engine & wgpu Pipeline Integration
**Goal:** Hook up the headless image parser (`rapidraw-core`) and graphics pipeline (`rapidraw-pipeline`) to feed actual image data into the GPUI window.

- [ ] **Step 2.1:** Implement RAW parsing and non-destructive math routines inside `rapidraw-core`.
- [ ] **Step 2.2:** Set up `wgpu` buffers and texture cache mapping in `rapidraw-pipeline` to bypass IPC bottlenecks.
- [ ] **Step 2.3:** Bind the texture allocation to a custom GPUI canvas element via `cx.paint_image`.
- [ **Verification:** ] Verify that a sample high-resolution image decodes headlessly, uploads to GPU memory, and renders smoothly inside the GPUI window canvas.

---

## Phase 3: Mobile Cross-Platform Extension (`gpui-mobile` + `gpui-kit`)
**Goal:** Extend the core architecture to iOS and Android early, ensuring the UI abstraction and pipeline remain completely portable across desktop and mobile.

- [ ] **Step 3.1:** Introduce conditional target compilation flags for mobile embedded entrypoints (`gpui_kit::is_mobile()`).
- [ ] **Step 3.2:** Configure iOS (Xcode/Metal) and Android (Gradle/Vulkan via `itsbalamurali/gpui-mobile`) compilation targets.
- [ ] **Step 3.3:** Adapt touch-handling mechanics (pinch-to-zoom, momentum scrolling, touch-friendly UI layouts).
- [ **Verification:** ] Compile and deploy the application to a physical iOS device and Android device; verify the image canvas renders and responds correctly to touch gestures inside native mobile containers.

---

## Phase 4: Non-Destructive Editing Controls & Interactivity
**Goal:** Build out core editing features, wiring asynchronous background worker threads to keep UI frame rates locked at 120Hz.

- [ ] **Step 4.1:** Build native slider and toggle components using `gpui-kit` primitives.
- [ ] **Step 4.2:** Route slider adjustments asynchronously via background worker channels so pixel updates happen off the main UI thread.
- [ ] **Step 4.3:** Implement smooth mouse/touch pan, zoom, and crop bounding box selection.
- [ **Verification:** ] Rapidly drag exposure and tone-mapping sliders while panning a 45-megapixel image; verify zero dropped frames and a silky-smooth UI response.

---

## Phase 5: Extended Goal — Cloud AI Inpainting & Advanced Integrations
**Goal:** Integrate remote AI endpoints (such as Google AI Studio/Gemini imaging models) as an optional, asynchronous enhancement layer.

- [ ] **Step 5.1:** Add an API key settings panel and secure credential storage.
- [ ] **Step 5.2:** Implement an async network layer to package masked crop selections and ship them to Gemini endpoints.
- [ ] **Step 5.3:** Handle payload responses, dynamically updating the local texture cache with generated inpaint results.
- [ **Verification:** ] Execute an inpainting request on a selected canvas region; verify background network calls complete successfully without locking up local pan/zoom responsiveness.

---

## Measurable Goals: Why GPUI-Kit Outperforms Web-Hybrid (Tauri/React)

| Metric | Tauri / React (Previous) | Pure Rust GPUI-Kit (Target) | Measured Proof Value |
| :--- | :--- | :--- | :--- |
| **Idle RAM Usage** | ~350 MB - 500 MB (Chromium overhead) | **< 50 MB** | Slashes baseline memory footprint by ~85%, preserving battery life for field photographers. |
| **Idle CPU Utilization** | 1% - 3% baseline background ticks | **0%** | Drops CPU activity to absolute zero when static. |
| **Pixel Rendering Path** | Slow JSON/Binary IPC bridge ➔ Webview DOM | **Zero-Copy Direct GPU Texture Interop** | Eliminates serialization latency; pixels paint straight into a GPU texture buffer. |
| **Cross-Platform Footprint** | Separate desktop and mobile web codebases | **Unified Pure-Rust Codebase (Desktop + iOS + Android)** | Shares core logic and UI primitives across all platforms without heavy JavaScript runtime baggage. |




---------------
below is the toml from rapidraw, added here for reference on crates it is currently using
[package]
name = "RapidRAW"
description = "A blazingly-fast, non-destructive, and GPU-accelerated RAW image editor built with performance in mind."
authors = ["Timon Käch"]
edition = "2024"
rust-version = "1.98"

[lib]
name = "rapidraw_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[features]
default = []
tethering = ["dep:gphoto2"]

[dependencies]
tauri = { version = "2.12", features = [ "macos-private-api", "rustls-tls", "protocol-asset" ] }
tauri-plugin-dialog = "2.7.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
image = "0.25.10"
png = "0.18.1"
tiff = "0.11.3"
base64 = "0.23"
tauri-plugin-fs = "2.5.1"
rayon = "1.12.0"
blake3 = "1.8.7"
wgpu = "29.0" # Downgraded to prevent P3 color shifts on Apple devices
pollster = "1.0"
bytemuck = { version = "1.25", features = ["derive"] }
anyhow = "1.0.104"
kamadak-exif = "0.6.1"
rawler = { git = "https://github.com/CyberTimon/RapidRAW-DngLab.git" }
tokio = { version = "1", features = ["full"] }
uuid = { version = "1.25", features = ["v4", "serde"] }
walkdir = "2.5.0"
imageproc = "0.27.0"
lsdetect = "0.3.0"
ort = { version = "=2.0.0-rc.10", features = ["ndarray", "load-dynamic"] }
ndarray = "0.16"
tauri-plugin-process = "2.3.1"
tauri-plugin-os = "2.3.2"
log = "0.4"
fern = "0.7"
little_exif = "0.6.23"
crc32fast = "1.5.0"
chrono = "0.4"
tokenizers = { version = "0.23.1", default-features = false, features = ["onig"] }
once_cell = "1.21"
futures = "0.3.34"
sha2 = "0.11"
nalgebra = "0.35.0"
hex = "0.4"
rand = "0.10"
tauri-plugin-shell = "2.3.5"
tempfile = "3.27.0"
# no_thp: MADV_HUGEPAGE causes multi-second compaction stalls on fragmented Linux systems.
mimalloc = { version = "0.1.52", features = ["no_thp"] }
image_hasher = "3.1.1"
regex = "1.13.1"
memmap2 = "0.9.11"
half = { version = "2.7.1", features = ["bytemuck"] }
glam = "0.33.5"
quick-xml = { version = "0.41", features = ["serialize"] }
fuzzy-matcher = "0.3.7"
image-hdr = { version = "0.6.0", default-features = false }
mozjpeg-rs = "0.9.2"
webp = "0.3"
jxl-oxide = { version = "0.12.6", features = ["image"] }
jxl-encoder = "0.3.1"
filetime = "0.2"
libc = "0.2.189"
rgb = "0.8.53"
imgref = "1.12.2"
sysinfo = "0.39.6"
tauri-plugin-http = "2.8.0"
tauri-plugin-store = "2.5.0"

[target.'cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))'.dependencies]
trash = "5.2.6"
keyring-core = "1"
tauri-plugin-single-instance = "2.4.3"
reqwest = { version = "0.13", default-features = false, features = ["json", "multipart", "rustls"] }
tauri-plugin-clerk = "0.1.1"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.62", features = ["Win32_Graphics_Dxgi", "Win32_System_Threading"] }
windows-native-keyring-store = "1"

[target.'cfg(any(target_os = "macos", target_os = "linux"))'.dependencies]
gphoto2 = { git = "https://github.com/CyberTimon/libgphoto2-rs", optional = true }

[target.'cfg(target_os = "macos")'.dependencies]
apple-native-keyring-store = { version = "1", features = ["keychain"] }
objc = "0.2"
objc2 = "0.6"
objc2-foundation = { version = "0.3.2", features = [
    "NSData",
    "NSDictionary",
    "NSString",
    "NSURL",
    "NSValue",
    "NSArray",
] }
objc2-core-foundation = { version = "0.3.2", features = ["CFCGTypes"] }
objc2-core-graphics = { version = "0.3.2", features = ["CGColorSpace"] }
objc2-core-image = { version = "0.3.2", features = [
    "CIContext",
    "CIFilter",
    "CIImage",
    "CIRAWFilter",
] }

[target.'cfg(target_os = "android")'.dependencies]
android-native-keyring-store = "1"
keyring-core = "1"
ndk-context = "0.1"
jni = "0.21"
jni22 = { package = "jni", version = "0.22" }
rustls-platform-verifier = "0.7"
reqwest = { version = "0.13", default-features = false, features = ["json", "multipart", "rustls"] }
include_dir = "0.7.4"

[target.'cfg(target_os = "linux")'.dependencies]
zbus-secret-service-keyring-store = { version = "1", features = ["rt-tokio-crypto-rust"] }
gtk = "0.18.2"
webkit2gtk = "=2.0.2"
tauri-plugin-wayland-nvidia-quirk = "0.1.0"
wayland-client = "0.31"
wayland-backend = { version = "0.3", features = ["client_system"] }
wayland-protocols = { version = "0.32", features = ["client"] }

[build-dependencies]
tauri-build = { version = "2.6", features = [] }
sha2 = "0.11"
reqwest = { version = "0.13", default-features = false, features = ["blocking", "rustls"] }
hex = "0.4"

[profile.dev]
opt-level = 2
debug = 1
incremental = true

[profile.dev.package."*"]
opt-level = 3

[profile.release]
codegen-units = 1
lto = true
strip = true

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(feature, values("cargo-clippy"))'] }
