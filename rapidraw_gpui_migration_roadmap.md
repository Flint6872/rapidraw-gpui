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
