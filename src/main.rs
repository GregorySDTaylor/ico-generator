mod config_constants;
mod icosahedron;

use std::array::from_fn;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use config_constants::*;
use icosahedron::*;

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
        .add_systems(Update, draw_debug)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let trile_handle: Handle<Image> = asset_server.load("land_both_corners.png");

    let up_options = vec![Deltille {
        image_handle: trile_handle.clone(),
        flip_x: false,
        flip_y: false,
        sockets: Sockets::Up {
            nw: "".to_string(),
            ne: "".to_string(),
            s: "".to_string(),
        },
    }];

    let down_options = vec![Deltille {
        image_handle: trile_handle.clone(),
        flip_x: false,
        flip_y: true,
        sockets: Sockets::Down {
            n: "".to_string(),
            se: "".to_string(),
            sw: "".to_string(),
        },
    }];

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

fn draw_debug(mut gizmos: Gizmos, precalculated_coordinates: Res<PreCalculatedCoordinates>) {
    draw_pixel_grid(&mut gizmos);
    draw_icoface_outlines(&mut gizmos, &precalculated_coordinates);
}

fn draw_pixel_grid(gizmos: &mut Gizmos) {
    for x in 0..WINDOW_GRID_WIDTH {
        gizmos.line_2d(
            Vec2::new(x as f32, 0.0),
            Vec2::new(x as f32, WINDOW_GRID_HEIGHT as f32),
            Color::DARK_GRAY,
        );
    }
    for y in 0..WINDOW_GRID_HEIGHT {
        gizmos.line_2d(
            Vec2::new(0.0, y as f32),
            Vec2::new(WINDOW_GRID_WIDTH as f32, y as f32),
            Color::DARK_GRAY,
        );
    }
}

fn draw_icoface_outlines(
    gizmos: &mut Gizmos,
    precalculated_coordinates: &Res<PreCalculatedCoordinates>,
) {
    for icoface_coordinates in precalculated_coordinates.all_ico_face_coordinates.iter() {
        origins_for_icoface_coordinates(icoface_coordinates.x, icoface_coordinates.y)
                .iter()
                .for_each(|origin| {
                    if icoface_coordinates.y % 2 == 0 {
                        draw_deltille_subdivisions_up(
                            origin,
                            Color::GRAY,
                            gizmos,
                            precalculated_coordinates,
                        );
                        draw_triangle(origin, &Orientation::Up, FACE_GRID_WIDTH as f32, Color::WHITE, gizmos);
                    } else {
                        draw_deltille_subdivisions_down(
                            origin,
                            Color::GRAY,
                            gizmos,
                            precalculated_coordinates,
                        );
                        draw_triangle(origin, &Orientation::Down, FACE_GRID_WIDTH as f32, Color::WHITE, gizmos);
                    }
                })
    }
}

const ONE_TWO_ROW_ORIGIN_X: f32 = GAP_GRID_WIDTH as f32 + (FACE_GRID_WIDTH as f32 * 0.5);
const THREE_FOUR_ROW_ORIGIN_X: f32 = (GAP_GRID_WIDTH as f32 * 2.0) + FACE_GRID_WIDTH as f32;
const ICOFACE_ROW_ORIGINS: [Vec2; 4] = [
    Vec2::new(
        ONE_TWO_ROW_ORIGIN_X,
        FACE_GRID_HEIGHT as f32 + FACE_GRID_HEIGHT_HALF,
    ),
    Vec2::new(
        ONE_TWO_ROW_ORIGIN_X,
        FACE_GRID_HEIGHT as f32 - FACE_GRID_HEIGHT_HALF,
    ),
    Vec2::new(THREE_FOUR_ROW_ORIGIN_X, FACE_GRID_HEIGHT_HALF),
    Vec2::new(
        THREE_FOUR_ROW_ORIGIN_X,
        WINDOW_GRID_HEIGHT as f32 - FACE_GRID_HEIGHT_HALF,
    ),
];

fn origins_for_icoface_coordinates(x: usize, y: usize) -> Vec<Vec2> {
    let mut origins: Vec<Vec2> = Vec::with_capacity(if y < 2 { 1 } else { 2 });
    let row_origin = ICOFACE_ROW_ORIGINS[y];
    let first_origin = Vec2::new(
        row_origin.x + (x as f32 * (FACE_GRID_WIDTH as f32 + GAP_GRID_WIDTH as f32 * 2.0)),
        row_origin.y,
    );
    origins.push(first_origin);
    // duplicate to simulate wrapping
    if y > 1 && x == 4 {
        origins.push(Vec2::new(
            first_origin.x - WINDOW_GRID_WIDTH as f32,
            first_origin.y,
        ));
    }
    return origins;
}

fn draw_triangle(
    origin: &Vec2,
    orientation: &Orientation,
    width: f32,
    color: Color,
    gizmos: &mut Gizmos,
) {
    let half_width = width / 2.0;
    let half_height = width * SQRT_0_POINT_75 / 2.0;
    match orientation {
        Orientation::Up => {
            let left = Vec2::new(origin.x - half_width, origin.y - half_height);
            let right = Vec2::new(origin.x + half_width, origin.y - half_height);
            let top = Vec2::new(origin.x, origin.y + half_height);
            gizmos.line_2d(left, right, color);
            gizmos.line_2d(left, top, color);
            gizmos.line_2d(top, right, color);
        }
        Orientation::Down => {
            let left = Vec2::new(origin.x - half_width, origin.y + half_height);
            let right = Vec2::new(origin.x + half_width, origin.y + half_height);
            let down = Vec2::new(origin.x, origin.y - half_height);
            gizmos.line_2d(left, right, color);
            gizmos.line_2d(left, down, color);
            gizmos.line_2d(down, right, color);
        }
    }
}

fn offset_for_deltille_component_up_icoface(coordinates: &DeltilleCoordinates) -> Vec2 {
    let row_size = (coordinates.y + 2) / 2;
    let x_offset = -(row_size as f32 - 1.0) * DELTILLE_GRID_WIDTH_HALF
        + (DELTILLE_GRID_WIDTH as f32 * coordinates.x as f32);
    let visual_row_depth = (coordinates.y + 1) / 2;
    let y_offset = FACE_GRID_HEIGHT_HALF
        - DELTILLE_GRID_HEIGHT_HALF
        - (visual_row_depth as f32 * DELTILLE_GRID_HEIGHT as f32);
    return Vec2::new(x_offset, y_offset);
}

fn offset_for_deltille_component_down_icoface(coordinates: &DeltilleCoordinates) -> Vec2 {
    let row_size = FACE_DELTILLE_WIDTH - (coordinates.y + 1) / 2;
    let x_offset = -(row_size as f32 - 1.0) * DELTILLE_GRID_WIDTH_HALF
        + (DELTILLE_GRID_WIDTH as f32 * coordinates.x as f32);
    let visual_row_depth = coordinates.y / 2;
    let y_offset = FACE_GRID_HEIGHT_HALF
        - DELTILLE_GRID_HEIGHT_HALF
        - (visual_row_depth as f32 * DELTILLE_GRID_HEIGHT as f32);
    return Vec2::new(x_offset, y_offset);
}

fn draw_deltille_subdivisions_up(
    origin: &Vec2,
    color: Color,
    gizmos: &mut Gizmos,
    precalculated_coordinates: &Res<PreCalculatedCoordinates>,
) {
    for deltille_coordinates in precalculated_coordinates.all_up_deltille_coordinates.iter() {
        let offset = offset_for_deltille_component_up_icoface(deltille_coordinates);
        draw_triangle(
            &Vec2::new(origin.x + offset.x, origin.y + offset.y),
            if deltille_coordinates.y % 2 == 0 {
                &Orientation::Up
            } else {
                &Orientation::Down
            },
            DELTILLE_GRID_WIDTH as f32,
            color,
            gizmos,
        );
    }
}

fn draw_deltille_subdivisions_down(
    origin: &Vec2,
    color: Color,
    gizmos: &mut Gizmos,
    precalculated_coordinates: &Res<PreCalculatedCoordinates>,
) {
    for deltille_coordinates in precalculated_coordinates
        .all_down_deltille_coordinates
        .iter()
    {
        let offset = offset_for_deltille_component_down_icoface(deltille_coordinates);
        draw_triangle(
            &Vec2::new(origin.x + offset.x, origin.y + offset.y),
            if deltille_coordinates.y % 2 == 0 {
                &Orientation::Down
            } else {
                &Orientation::Up
            },
            DELTILLE_GRID_WIDTH as f32,
            color,
            gizmos,
        );
    }
}

// just for fun! replace this with actual sprite choices
fn place_deltille_sprites(
    mut commands: Commands,
    icosahedron: Res<Icosahedron>,
    precalculated_coordinates: Res<PreCalculatedCoordinates>,
) {
    for (icoface_y, row) in icosahedron.faces.iter().enumerate() {
        for (icoface_x, icoface) in row.iter().enumerate() {
            origins_for_icoface_coordinates(icoface_x, icoface_y)
                .iter()
                .for_each(|origin| {
                    if icoface_y % 2 == 0 {
                        place_deltille_sprites_up(
                            &mut commands,
                            icoface,
                            origin,
                            &precalculated_coordinates,
                        );
                    } else {
                        place_deltille_sprites_down(
                            &mut commands,
                            icoface,
                            origin,
                            &precalculated_coordinates,
                        );
                    }
                })
        }
    }
}

fn place_deltille_sprites_up(
    commands: &mut Commands,
    icoface: &IcoFace,
    origin: &Vec2,
    precalculated_coordinates: &Res<PreCalculatedCoordinates>,
) {
    for deltille_coordinates in precalculated_coordinates.all_up_deltille_coordinates.iter() {
        let offset = offset_for_deltille_component_up_icoface(deltille_coordinates);
        let image_handle = icoface
            .get_deltille_options_at(deltille_coordinates)
            .and_then(|options| options.first())
            .unwrap()
            .image_handle
            .clone();
        commands.spawn(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(origin.x + offset.x, origin.y + offset.y, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(
                    DELTILLE_GRID_WIDTH as f32,
                    DELTILLE_GRID_HEIGHT as f32,
                )),
                flip_y: deltille_coordinates.y % 2 != 0,
                ..default()
            },
            ..default()
        });
    }
}

fn place_deltille_sprites_down(
    commands: &mut Commands,
    icoface: &IcoFace,
    origin: &Vec2,
    precalculated_coordinates: &Res<PreCalculatedCoordinates>,
) {
    for deltille_coordinates in precalculated_coordinates
        .all_down_deltille_coordinates
        .iter()
    {
        let offset = offset_for_deltille_component_down_icoface(deltille_coordinates);
        let image_handle = icoface
            .get_deltille_options_at(deltille_coordinates)
            .unwrap()
            .first()
            .unwrap()
            .image_handle
            .clone();
        commands.spawn(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(origin.x + offset.x, origin.y + offset.y, 0.0),
                ..default()
            },
            sprite: Sprite {
                flip_y: deltille_coordinates.y % 2 == 0,
                ..default()
            },
            ..default()
        });
    }
}
