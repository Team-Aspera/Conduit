# Troubleshooting & Known Issues

## 1. Window Resize Lag and wgpu Warnings
**Symptoms:**
- Fast window resizing causes significant lag/stickiness.
- Logs flooded with: `WARN wgpu_hal::vulkan::conv: Unrecognized present mode 1000361000`.

**Cause:**
- The code `1000361000` corresponds to `VK_PRESENT_MODE_FIFO_RELAXED_KHR`.
- Some Linux drivers/compositors (Vulkan) provide this mode, but the version of `wgpu` in `iced 0.12` might not recognize it, leading to continuous fallback/negotiation and synchronous logging on the main thread.

**Attempted Fixes & Results:**
- **Silencing logs:** (`wgpu=error`) Reduces I/O pressure but "stickiness" may remain if the underlying driver negotiation is slow.
- **Forcing standard FIFO:** (`WGPU_PRESENT_MODE=fifo`) Aims to avoid the unrecognized mode.
- **Immediate Mode:** (`WGPU_PRESENT_MODE=immediate`) Theoretically the most responsive as it disables VSync.

## 2. Panic: `Quad with non-normal height!`
**Symptoms:**
- Application crashes during window resizing.
- Panic message: `thread 'main' panicked at ... iced_tiny_skia-0.12.1/src/backend.rs:162:17: Quad with non-normal height!`.

**Cause:**
- This occurs when using the **Software Renderer (tiny_skia)**.
- If `wgpu` fails to initialize (e.g., due to invalid environment variables like `WGPU_BACKEND=gl` on a system with poor GL support), `iced` falls back to `tiny_skia`.
- `tiny_skia` has a bug where containers with rounded corners or borders panic if their height becomes 0 or negative during rapid resizing.

**Solution/Workaround:**
- Ensure hardware acceleration is working (don't force unstable backends).
- Avoid forcing `WGPU_BACKEND` unless certain of system compatibility.
