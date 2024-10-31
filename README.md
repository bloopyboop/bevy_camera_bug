This repository demonstrates a bug in bevy 0.14.2.

## My system
- OS: Linux
- Display Server: Wayland
- Compositor: Hyprland (sway-wm based)
- `AdapterInfo { name: "NVIDIA GeForce GTX 1080", vendor: 4318, device: 7040, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "560.35.03", backend: Vulkan }`

Bug happens with X and i3wm too, with or without a compositor.

## Required parts for the bug to trigger
- 2D Camera
  - HDR enabled
  - BloomSettings component with an intensity greater than zero
- System changing the size of the camera's viewport every frame

## Observed effects
- Debug: FPS will drain continuously.
- Release: FPS do not drop each frame, instead only when the viewport's X size is large and Y size small. The FPS returns to normal for small X size and large Y size.
- Debug and Release: bevy will crash when the viewport is very long and thin (error message is commented in 'main.rs')

This does not occur when the area remains constant, even if the
viewport is moving.
Resizing the window will restore the FPS, but does not
permanently fix the issue.
Comments in 'main.rs' elaborate relevant details.

## To reproduce
- Have rust nightly toolchain installed (if you want to test stable, remove rust-toolchain.toml)
- Clone the repo
- `cargo run`

Press SPACE in the application to cycle through four test cases:
- Static viewport -> No effect on FPS
- Continuously resizing viewport -> FPS continuously drains
- Moving viewport with constant area -> No effect on FPS
- Setting viewport dimensions to be very long and thin -> Crash
