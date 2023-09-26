mod config_constants;
mod icosahedron;
mod graphics;

use std::array::from_fn;
use std::fs;

use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::{WindowResolution, PrimaryWindow};
use config_constants::*;
use icosahedron::*;
use graphics::*;

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
        .insert_resource(PreCalculatedCoordinates::generate())
        .add_systems(Startup, setup)
        .add_systems(PostStartup, place_deltille_sprites)
        .add_systems(PostStartup, screenshot.after(place_deltille_sprites))
        .add_systems(Update, draw_debug)
        .run();
}

fn screenshot(main_window: Query<Entity, With<PrimaryWindow>>, mut screenshot_manager: ResMut<ScreenshotManager>) {
    fs::create_dir_all("generated").unwrap();
    screenshot_manager.save_screenshot_to_disk(main_window.single(), "assets/generated/screenshot.png").unwrap();
}

fn up_options_from(deltille_inputs: &Vec<Deltille>) -> Vec<Deltille> {
    let mut deltilles = Vec::with_capacity(deltille_inputs.len() * 2);
    for deltille in deltille_inputs.iter() {
        deltilles.push(deltille.clone());
        let mut flipped = deltille.clone();
        flipped.flip_x = true;
        deltilles.push(flipped);
    }
    return deltilles;
}

fn down_options_from(deltille_inputs: &Vec<Deltille>) -> Vec<Deltille> {
    let mut deltilles = Vec::with_capacity(deltille_inputs.len() * 2);
    for deltille in deltille_inputs.iter() {
        let mut clone = deltille.clone();
        clone.flip_y = true;
        deltilles.push(clone);
        let mut flipped = deltille.clone();
        flipped.flip_x = true;
        flipped.flip_y = true;
        deltilles.push(flipped);
    }
    return deltilles;
}

fn setup(mut commands: Commands) {
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

    let deltille_json = fs::read_to_string("assets/deltilles.json").unwrap_or("[]".to_string());
    let deltilles: Vec<Deltille> = serde_json::from_str(&deltille_json).unwrap();

    let up_options = up_options_from(&deltilles);
    let down_options = down_options_from(&deltilles);

    let up_icoface = IcoFace::with_initial_options(Orientation::Up, &up_options, &down_options);
    let down_icoface = IcoFace::with_initial_options(Orientation::Down, &up_options, &down_options);

    commands.insert_resource(Icosahedron {
        faces: [
            from_fn(|_| up_icoface.clone()),
            from_fn(|_| down_icoface.clone()),
            from_fn(|_| up_icoface.clone()),
            from_fn(|_| down_icoface.clone()),
        ],
    });
}
