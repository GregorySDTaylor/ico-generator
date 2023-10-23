mod config_constants;
mod graphics;
mod icosahedron;
mod wave_function_collapse;

use std::collections::HashSet;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use config_constants::*;
use icosahedron::Icosahedron;
use graphics::*;
// use wave_function_collapse::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Icosphere Texture Generator".to_string(),
                        resolution: WindowResolution::new(
                            WINDOW_GRID_WIDTH as f32 * VIEW_SCALE,
                            WINDOW_GRID_HEIGHT as f32 * VIEW_SCALE,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Icosahedron::new(&[HashSet::new(), HashSet::new()]))
        // .insert_resource(WfcState::new())
        // .add_systems(Update, iterate_wfc.run_if(not_yet_complete))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_debug)
        .run();
}

fn setup(mut commands: Commands, mut gizmoConfig: ResMut<GizmoConfig>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0 / VIEW_SCALE,
            ..default()
        },
        transform: Transform::from_xyz(
            WINDOW_GRID_WIDTH as f32 / 2.0,
            WINDOW_GRID_HEIGHT as f32 / 2.0,
            1.0,
        ),
        ..default()
    });
    gizmoConfig.line_width = 1.5;
}
