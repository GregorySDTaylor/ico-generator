mod icosahedron;

use std::array::from_fn;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use icosahedron::*;

const VIEW_SCALE: f32 = 9.0;
const DELTILLE_GRID_WIDTH: usize = 16;
const GAP_GRID_WIDTH_HALF: usize = 1;
const FACE_DELTILLE_WIDTH: usize = 3;

const GAP_GRID_WIDTH: usize = GAP_GRID_WIDTH_HALF * 2;
const SQRT_0_POINT_75: f32 = 0.86602540378443864676372317075293;
const DELTILLE_GRID_HEIGHT: usize = (DELTILLE_GRID_WIDTH as f32 * SQRT_0_POINT_75) as usize + 1;
const DELTILLE_GRID_HEIGHT_HALF: f32 = DELTILLE_GRID_HEIGHT as f32 / 2.0;
const DELTILLE_GRID_WIDTH_HALF: f32 = DELTILLE_GRID_WIDTH as f32 / 2.0;
const FACE_GRID_WIDTH: usize = FACE_DELTILLE_WIDTH as usize * DELTILLE_GRID_WIDTH as usize;
// faces are slightly taller to accomodate imperfect deltille pixel heights
const FACE_GRID_HEIGHT: usize = DELTILLE_GRID_HEIGHT as usize * FACE_DELTILLE_WIDTH as usize;
const FACE_GRID_HEIGHT_HALF: f32 = FACE_GRID_HEIGHT as f32 * 0.5;
const WINDOW_GRID_WIDTH: usize = 5 * (FACE_GRID_WIDTH as usize + GAP_GRID_WIDTH as usize * 2);
const WINDOW_GRID_HEIGHT: usize = 2 * FACE_GRID_HEIGHT as usize;

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

    let up_deltille = Deltille {
        image_handle: trile_handle.clone(),
        flip_x: false,
        sockets: Sockets::Up {
            nw: "".to_string(),
            ne: "".to_string(),
            s: "".to_string(),
        },
    };

    let down_deltille = Deltille {
        image_handle: trile_handle.clone(),
        flip_x: false,
        sockets: Sockets::Down {
            n: "".to_string(),
            se: "".to_string(),
            sw: "".to_string(),
        },
    };

    let up_icoface = IcoFace {
        deltilles: up_face_deltilles_initial(
            vec![up_deltille.clone()],
            vec![down_deltille.clone()],
        ),
    };

    let down_icoface = IcoFace {
        deltilles: down_face_deltilles_initial(
            vec![up_deltille.clone()],
            vec![down_deltille.clone()],
        ),
    };

    let icosahedron = Icosahedron {
        faces: [
            from_fn(|_| up_icoface.clone()),
            from_fn(|_| down_icoface.clone()),
            from_fn(|_| up_icoface.clone()),
            from_fn(|_| down_icoface.clone()),
        ],
    };

    commands.spawn(icosahedron);
}

fn up_face_deltilles_initial(
    up_deltille_options: Vec<Deltille>,
    down_deltille_options: Vec<Deltille>,
) -> Vec<Vec<Vec<Deltille>>> {
    let rowcount = FACE_DELTILLE_WIDTH as usize * 2 - 1;
    let mut rows: Vec<Vec<Vec<Deltille>>> = Vec::with_capacity(rowcount);
    for i in 0..rowcount {
        let row_size = (2 + i) / 2;
        let mut row: Vec<Vec<Deltille>> = Vec::with_capacity(row_size);
        row.resize_with(row_size, || {
            if i % 2 == 0 {
                up_deltille_options.clone()
            } else {
                down_deltille_options.clone()
            }
        });
        rows.push(row);
    }
    return rows;
}

fn down_face_deltilles_initial(
    up_deltille_options: Vec<Deltille>,
    down_deltille_options: Vec<Deltille>,
) -> Vec<Vec<Vec<Deltille>>> {
    let rowcount = FACE_DELTILLE_WIDTH as usize * 2 - 1;
    let mut rows: Vec<Vec<Vec<Deltille>>> = Vec::with_capacity(rowcount);
    for i in (0..rowcount).rev() {
        let row_size = (2 + i) / 2;
        let mut row: Vec<Vec<Deltille>> = Vec::with_capacity(row_size);
        row.resize_with(row_size, || {
            if i % 2 == 0 {
                down_deltille_options.clone()
            } else {
                up_deltille_options.clone()
            }
        });
        rows.push(row);
    }
    return rows;
}

fn draw_debug(mut gizmos: Gizmos, icosahedron_query: Query<&Icosahedron>) {
    let icosahedron = icosahedron_query.get_single().unwrap();
    draw_pixel_grid(&mut gizmos);
    draw_icoface_outlines(&mut gizmos, icosahedron);
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

fn draw_icoface_outlines(gizmos: &mut Gizmos, icosahedron: &Icosahedron) {
    for (icoface_y, row) in icosahedron.faces.iter().enumerate() {
        for (icoface_x, icoface) in row.iter().enumerate() {
            origins_for_icoface_coordinates(icoface_x, icoface_y)
                .iter()
                .for_each(|origin| {
                    if icoface_y % 2 == 0 {
                        draw_deltille_subdivisions_up(icoface, origin, Color::GRAY, gizmos);
                        draw_triangle_up(origin, FACE_GRID_WIDTH as f32, Color::WHITE, gizmos);
                    } else {
                        draw_deltille_subdivisions_down(icoface, origin, Color::GRAY, gizmos);
                        draw_triangle_down(origin, FACE_GRID_WIDTH as f32, Color::WHITE, gizmos);
                    }
                })
        }
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

fn draw_triangle_up(origin: &Vec2, width: f32, color: Color, gizmos: &mut Gizmos) {
    let half_width = width / 2.0;
    let half_height = width * SQRT_0_POINT_75 / 2.0;
    let left = Vec2::new(origin.x - half_width, origin.y - half_height);
    let right = Vec2::new(origin.x + half_width, origin.y - half_height);
    let top = Vec2::new(origin.x, origin.y + half_height);
    gizmos.line_2d(left, right, color);
    gizmos.line_2d(left, top, color);
    gizmos.line_2d(top, right, color);
}

fn draw_triangle_down(origin: &Vec2, width: f32, color: Color, gizmos: &mut Gizmos) {
    let half_width = width / 2.0;
    let half_height = width * SQRT_0_POINT_75 / 2.0;
    let left = Vec2::new(origin.x - half_width, origin.y + half_height);
    let right = Vec2::new(origin.x + half_width, origin.y + half_height);
    let down = Vec2::new(origin.x, origin.y - half_height);
    gizmos.line_2d(left, right, color);
    gizmos.line_2d(left, down, color);
    gizmos.line_2d(down, right, color);
}

fn offset_for_deltille_component_up_icoface(x: usize, y: usize) -> Vec2 {
    let row_size = (y + 2) / 2;
    let x_offset = -(row_size as f32 - 1.0) * DELTILLE_GRID_WIDTH_HALF
        + (DELTILLE_GRID_WIDTH as f32 * x as f32);
    let visual_row_depth = (y + 1) / 2;
    let y_offset = FACE_GRID_HEIGHT_HALF
        - DELTILLE_GRID_HEIGHT_HALF
        - (visual_row_depth as f32 * DELTILLE_GRID_HEIGHT as f32);
    return Vec2::new(x_offset, y_offset);
}

fn offset_for_deltille_component_down_icoface(x: usize, y: usize) -> Vec2 {
    let row_size = FACE_DELTILLE_WIDTH - (y + 1) / 2;
    let x_offset = -(row_size as f32 - 1.0) * DELTILLE_GRID_WIDTH_HALF
        + (DELTILLE_GRID_WIDTH as f32 * x as f32);
    let visual_row_depth = y / 2;
    let y_offset = FACE_GRID_HEIGHT_HALF
        - DELTILLE_GRID_HEIGHT_HALF
        - (visual_row_depth as f32 * DELTILLE_GRID_HEIGHT as f32);
    return Vec2::new(x_offset, y_offset);
}

fn draw_deltille_subdivisions_up(
    icoface: &IcoFace,
    origin: &Vec2,
    color: Color,
    gizmos: &mut Gizmos,
) {
    for (y, row) in icoface.deltilles.iter().enumerate() {
        if y % 2 == 0 {
            for x in 0..row.len() {
                let offset = offset_for_deltille_component_up_icoface(x, y);
                draw_triangle_up(
                    &Vec2::new(origin.x + offset.x, origin.y + offset.y),
                    DELTILLE_GRID_WIDTH as f32,
                    color,
                    gizmos,
                );
            }
        } else {
            for x in 0..row.len() {
                let offset = offset_for_deltille_component_up_icoface(x, y);
                draw_triangle_down(
                    &Vec2::new(origin.x + offset.x, origin.y + offset.y),
                    DELTILLE_GRID_WIDTH as f32,
                    color,
                    gizmos,
                );
            }
        }
    }
}

fn draw_deltille_subdivisions_down(
    icoface: &IcoFace,
    origin: &Vec2,
    color: Color,
    gizmos: &mut Gizmos,
) {
    for (y, row) in icoface.deltilles.iter().enumerate() {
        if y % 2 == 0 {
            for x in 0..row.len() {
                let offset = offset_for_deltille_component_down_icoface(x, y);
                draw_triangle_down(
                    &Vec2::new(origin.x + offset.x, origin.y + offset.y),
                    DELTILLE_GRID_WIDTH as f32,
                    color,
                    gizmos,
                );
            }
        } else {
            for x in 0..row.len() {
                let offset = offset_for_deltille_component_down_icoface(x, y);
                draw_triangle_up(
                    &Vec2::new(origin.x + offset.x, origin.y + offset.y),
                    DELTILLE_GRID_WIDTH as f32,
                    color,
                    gizmos,
                );
            }
        }
    }
}

// just for fun! replace this with actual sprite choices
fn place_deltille_sprites(mut commands: Commands, icosahedron_query: Query<&Icosahedron>) {
    let icosahedron = icosahedron_query.get_single().unwrap();
    for (icoface_y, row) in icosahedron.faces.iter().enumerate() {
        for (icoface_x, icoface) in row.iter().enumerate() {
            origins_for_icoface_coordinates(icoface_x, icoface_y)
                .iter()
                .for_each(|origin| {
                    if icoface_y % 2 == 0 {
                        place_deltille_sprites_up(&mut commands, icoface, origin);
                    } else {
                        place_deltille_sprites_down(&mut commands, icoface, origin);
                    }
                })
        }
    }
}

fn place_deltille_sprites_up(commands: &mut Commands, icoface: &IcoFace, origin: &Vec2) {
    for (y, row) in icoface.deltilles.iter().enumerate() {
        for (x, deltille_options) in row.iter().enumerate() {
            let offset = offset_for_deltille_component_up_icoface(x, y);
            let image_handle = deltille_options.first().unwrap().image_handle.clone();
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
                    flip_y: y % 2 != 0,
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn place_deltille_sprites_down(commands: &mut Commands, icoface: &IcoFace, origin: &Vec2) {
    for (y, row) in icoface.deltilles.iter().enumerate() {
        for (x, deltille_options) in row.iter().enumerate() {
            let offset = offset_for_deltille_component_down_icoface(x, y);
            let image_handle = deltille_options.first().unwrap().image_handle.clone();
            commands.spawn(SpriteBundle {
                texture: image_handle,
                transform: Transform {
                    translation: Vec3::new(origin.x + offset.x, origin.y + offset.y, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    flip_y: y % 2 == 0,
                    ..default()
                },
                ..default()
            });
        }
    }
}
