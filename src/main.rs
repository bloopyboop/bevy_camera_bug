use bevy::prelude::*;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::window::{WindowResolution, PresentMode, PrimaryWindow};
use bevy::render::camera::Viewport;

// FPS display, not relevant for the bug
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::dev_tools::fps_overlay::{FpsOverlayPlugin, FpsOverlayConfig};

// No relevant settings for the bug in here. Just sets up bevy.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1000., 1000.),
                    title: "camera_bug".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 32.0,
                    ..default()
                },
            },
        })
        .insert_resource( Animation(AnimationTag::Static) )
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            resize_camera_b_viewport,
            cycle_animation_type
        ))
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let bug_camera = commands.spawn(( // Camera which demonstrates the bug.
        Camera2dBundle {
            camera: Camera {
                hdr: true, // Required to be 'true' for the bug.
                order: 1, // NOT required, but higher than the other camera, so the viewport can be seen for testing.
                ..default() // Bug seems indifferent to every other setting.
            },
            ..default()
        },
        BloomSettings {
            intensity: 0.2, // Required to be greater zero for the bug.
            ..default() // Bug seems indifferent to every other setting.
        },
    ))
    .insert(AnimatedViewport)
    .id();

    // Creating some UI so above camera's viewport can be seen.
    // NOT required to trigger the bug.
    commands.spawn( NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::srgba(1.00, 0.65, 0.45, 1.0).into(),
        ..default()
    })
    .insert(TargetCamera(bug_camera));

    commands.spawn(( // Camera which is NOT required for the bug. It's only here to display the FPS overlay.
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
    ))
    .insert(IsDefaultUiCamera);
}

// For testing convenience. NOT required for the bug.
enum AnimationTag {
    Static,
    Resize,
    MovementOnly,
    ForceCrash,
}

// For testing convenience. NOT required for the bug.
#[derive(Resource)]
struct Animation(AnimationTag);

// For testing convenience. NOT required for the bug.
fn cycle_animation_type(
    input: Res<ButtonInput<KeyCode>>,
    mut animation: ResMut<Animation>,
) {
    if input.just_pressed(KeyCode::Space) {
        animation.0 = match animation.0 {
            AnimationTag::Static => AnimationTag::Resize,
            AnimationTag::Resize => AnimationTag::MovementOnly,
            AnimationTag::MovementOnly => AnimationTag::ForceCrash,
            AnimationTag::ForceCrash => AnimationTag::Static,
        }
    }
}

#[derive(Component)]
struct AnimatedViewport;

// Demonstration of the bug.
fn resize_camera_b_viewport(
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<&mut Camera, With<AnimatedViewport>>,
    time: Res<Time>,
    animation: Res<Animation>,
) {
    let window = window.single();
    let mut camera = camera.single_mut();

    // Animation parameters. Their specifics are irrelevant for the bug, except that they make sure
    // the viewport's dimensions are always valid.
    let t = (f32::sin(time.elapsed().as_secs_f32()) + 1.) / 2.; // Oscillates between 0 and 1.
    let size = window.physical_size().as_vec2();
    const MINIMAL_SIZE_PERCENTAGE_OF_WINDOW: f32 = 0.1;
    let minimal_size = size * MINIMAL_SIZE_PERCENTAGE_OF_WINDOW;

    match camera.viewport {
        None => camera.viewport = Some( Viewport { physical_size: [1,1].into(), ..default() } ),
        Some(ref mut viewport) => { match animation.0 {
            AnimationTag::Static => { // FPS is unaffected
                viewport.physical_position = minimal_size.as_uvec2();
                viewport.physical_size = (size - 2. * minimal_size).as_uvec2();
            }
            AnimationTag::Resize => { // FPS drops more each frame. Exact animation details do not matter.
                viewport.physical_position = minimal_size.as_uvec2();
                viewport.physical_size.x = f32::lerp(minimal_size.x, size.x-2.*minimal_size.x, t) as u32;
                viewport.physical_size.y = f32::lerp(size.y-2.*minimal_size.y, minimal_size.y, t) as u32;
            },
            AnimationTag::MovementOnly => { // FPS is unaffected if viewport area remains constant, even when moving
                viewport.physical_position.x = f32::lerp(minimal_size.x, size.x-2.*minimal_size.x, t) as u32;
                viewport.physical_position.y = f32::lerp(minimal_size.y, size.y-2.*minimal_size.y, t) as u32;
                viewport.physical_size = minimal_size.as_uvec2();
            }
            AnimationTag::ForceCrash => { // Crash when viewport dimensions are very thin, but very long.
                viewport.physical_position = minimal_size.as_uvec2();
                viewport.physical_size = ((size.x - 2. * minimal_size.x) as u32, 1).into();

                /* 
                This is the kind of error I get when this switch prong runs:

                ERROR wgpu::backend::wgpu_core: Handling wgpu errors as fatal by default    
                thread 'Compute Task Pool (4)' panicked at [REDACTED]/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wgpu-0.20.1/src/backend/wgpu_core.rs:2996:5:
                wgpu error: Validation Error

                Caused by:
                In Device::create_texture
                note: label = `bloom_texture`
                Dimension X value 35170 exceeds the limit of 32768

                */
            }
        }},
    }

}
