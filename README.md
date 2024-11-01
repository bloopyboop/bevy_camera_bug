This repository demonstrates a bug in multiple versions of bevy.

## Branches
- main: bevy 0.14.2
- bevy_main: bevy 0.15.0-dev, rev [#4656698](https://github.com/bevyengine/bevy/commit/46566980a6d69d2bd91505b6acd49ababa4d98f7)
- bevy_0.15.0-rc.2: bevy 0.15.0-rc.2

All branches demonstrate the bug.

## My system
- OS: Linux
- Display Server: Wayland
- Compositor: Hyprland (sway-wm based)
- `AdapterInfo { name: "NVIDIA GeForce GTX 1080", vendor: 4318, device: 7040, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "560.35.03", backend: Vulkan }`

Bug happens with X and i3wm too, with or without a compositor.

## Required parts for the bug to trigger
- 2D Camera
  - HDR enabled
  - Bloom component with an intensity greater than zero (BloomSettings when on main branch)
- System changing the size of the camera's viewport every frame

## Observed effects
- Debug: FPS will drain continuously.
- Release: FPS do not drop each frame, instead only when the viewport's X size is large and Y size small. The FPS returns to normal for small X size and large Y size.
- Debug and Release: bevy will crash when the viewport is very long and thin (error message is commented in 'main.rs')

None of the above bugs occur when the viewport's area remains constant, even if the
viewport is moving.
Resizing the window will restore the FPS, but does not
permanently fix the issue.
Comments in 'main.rs' elaborate relevant details.

## To reproduce
- Have rust nightly toolchain installed (if you want to test stable, remove rust-toolchain.toml)
- Clone the repo
- Checkout the branch for the bevy version you want to test:
  - bevy 0.14.2: `git checkout main`
  - bevy 0.15.0-dev: `git checkout bevy_main`
  - bevy 0.15.0-rc.2: `git checkout bevy_0.15.0-rc.2`
- `cargo run`

Press SPACE in the application to cycle through four test cases:
- Static viewport -> No effect on FPS
- Continuously resizing viewport -> FPS continuously drains
- Moving viewport with constant area -> No effect on FPS
- Setting viewport dimensions to be very long and thin -> Crash
