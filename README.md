This repository demonstrates a bug in bevy 0.14.2.

My system is Arch Linux on Wayland with Hyprland (sway-wm based) compositor. I encounter the bug with X, as well.
Here is my AdapterInfo: 
`AdapterInfo { name: "NVIDIA GeForce GTX 1080", vendor: 4318, device: 7040, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "560.35.03", backend: Vulkan }`

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
- Static viewport
- Continuously resizing viewport
- Moving viewport with constant area
- Forced crash by making the viewport very long and thin
