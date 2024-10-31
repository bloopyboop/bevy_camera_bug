This repository demonstrates a bug in bevy 0.14.2.

When a 2D camera has HDR and bloom enabled, resizing its viewport
such that the aspect ratio changes, will continuously drain FPS.

This does not occur when the area remains constant, even if the
viewport is moving.

Resizing the window will restore the FPS, but does not
permanently fix the issue.

Additionally, the program crashes when the viewport's dimensions
are very long and thin. The error message can be found in
'main.rs'. Comments in 'main.rs' go into detail.


Press SPACE in the application to cycle through four test cases:
    - Static viewport
    - Continuously resizing viewport
    - Moving viewport with constant area
    - Forced crash by making the viewport very long and thin
