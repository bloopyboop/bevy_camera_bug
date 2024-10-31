This repository demonstrates a bug in bevy 0.14.2.

My system:
- OS: Linux
- Display Server: Wayland
- Compositor: Hyprland (sway-wm based)
- `AdapterInfo { name: "NVIDIA GeForce GTX 1080", vendor: 4318, device: 7040, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "560.35.03", backend: Vulkan }`

Bug happens with X and i3wm too, with or without a compositor.

When a 2D camera has HDR and bloom enabled, resizing its viewport
such that the aspect ratio changes, will continuously drain FPS.
This does not occur when the area remains constant, even if the
viewport is moving.
Resizing the window will restore the FPS, but does not
permanently fix the issue.
Comments in 'main.rs' elaborate relevant details.

Additionally, the program crashes when the viewport's dimensions
are very long and thin. The error message can be found in
'main.rs'.

All of this only noticably happens in debug builds - Release
might still have the bug, but would require sensitive
benchmarking to detect. Even if it doesn't have the bug, its
existence in debug might point to a problematic bloom
implementation.


Press SPACE in the application to cycle through four test cases:
- Static viewport -> No effect on FPS
- Continuously resizing viewport -> FPS continuously drains
- Moving viewport with constant area -> No effect on FPS
- Forced crash by making the viewport very long and thin -> Crash
