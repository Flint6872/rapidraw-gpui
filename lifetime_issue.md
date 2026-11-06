error: lifetime may not live long enough

## GPUI Rust Pattern: Handling `&mut AsyncApp` in `cx.spawn()`

### Problem Overview
When using `cx.spawn()` in GPUI, passing an `async move` block directly with `&mut AsyncApp` causes a Rust lifetime compilation error:

error: lifetime may not live long enough
|
88 | |   |this: WeakEntity, async_cx: &mut AsyncApp| async move {
| |___________________________________________________^
| |   return type of closure () contains a lifetime '2
| |   let's call the lifetime of this reference '1
| |   returning this value requires that '1 must outlive '2
### Root Cause
1. `cx.spawn()` passes `async_cx` as a borrowed mutable reference (`&mut AsyncApp`).
2. An `async move` block taking `async_cx` directly attempts to move or capture the reference `&'1 mut AsyncApp` inside the generated `Future`.
3. When operations (like `.await` or `.update()`) occur inside the future, the Rust borrow checker cannot prove that the outer reference `'1` outlives the future's lifetime `'2`.

---

### Anti-Pattern (DO NOT DO THIS)
Do NOT pass `async move` directly as the `cx.spawn` closure body while expecting `&mut AsyncApp` to automatically move cleanly:

```rust
// ❌ WRONG: Borrowed reference trapped in async move block
cx.spawn(|this: WeakEntity<Self>, async_cx: &mut AsyncApp| async move {
    let processed = processing_task.await;
    this.update(async_cx, |this, cx| {
        // ...
    });
})
.detach();
Standard Fix Pattern (ALWAYS DO THIS)Clone async_cx into an owned instance outside the async move block before capturing it:Rust// ✅ CORRECT: Clone the borrowed reference into an owned instance
cx.spawn(|this: WeakEntity<Self>, async_cx: &mut AsyncApp| {
    // 1. Clone &mut AsyncApp to create an owned AsyncApp instance
    let mut async_cx = async_cx.clone();

    // 2. Move the owned AsyncApp into the async block
    async move {
        // Perform async work (awaiting tasks, prompt receivers, etc.)
        let processed = processing_task.await;

        // Pass &mut async_cx into update calls inside the async block
        this.update(&mut async_cx, |this, cx| {
            let rgba_img = image::RgbaImage::from_raw(
                processed.width,
                processed.height,
                processed.rgba_data,
            )
            .unwrap_or_else(|| image::RgbaImage::new(1, 1));

            let frame = image::Frame::new(rgba_img);
            this.state.render_image = std::sync::Arc::new(RenderImage::new(vec![frame]));
            this.is_processing = false;
            cx.notify();
        })
        .log_err();
    }
})
.detach();
Key Takeaways for GPUI Async CodeAsyncApp implements Clone.Cloned AsyncApp handles are lightweight reference-counted handles back to the main UI context loop.Outer closure takes &mut AsyncApp $\rightarrow$ Clone to local let mut async_cx = async_cx.clone(); $\rightarrow$ Open async move block $\rightarrow$ Pass &mut async_cx to .update().
