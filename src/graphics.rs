use bevy::prelude::{Color, Gizmos, Res, Vec2};

use crate::{
    config_constants::*,
    icosahedron::{Icosahedron, VerticalOrientation, IcoFace, DeltilleSlot},
};
// use crate::icosahedron::*;

pub fn draw_debug(mut gizmos: Gizmos, icosahedron: Res<Icosahedron>) {
    draw_pixel_grid(&mut gizmos);
    debug_icosahedron(&mut gizmos, icosahedron);
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

fn debug_icosahedron(gizmos: &mut Gizmos, icosahedron: Res<Icosahedron>) {
    for icoface in icosahedron.icofaces.iter() {
        debug_icoface(icoface, gizmos);
    }

    // for icoface_coordinates in precalculated_coordinates.all_ico_face_coordinates.iter() {
    //     let origin = origin_for_icoface_coordinates(icoface_coordinates.x, icoface_coordinates.y);
    //     if icoface_coordinates.y % 2 == 0 {
    //         draw_deltille_subdivisions_up(&origin, Color::GRAY, gizmos, precalculated_coordinates);
    //         draw_triangle(
    //             &origin,
    //             &VerticalOrientation::Up,
    //             FACE_GRID_WIDTH as f32,
    //             FACE_GRID_HEIGHT as f32,
    //             Color::WHITE,
    //             gizmos,
    //         );
    //     } else {
    //         draw_deltille_subdivisions_down(
    //             &origin,
    //             Color::GRAY,
    //             gizmos,
    //             precalculated_coordinates,
    //         );
    //         draw_triangle(
    //             &origin,
    //             &VerticalOrientation::Down,
    //             FACE_GRID_WIDTH as f32,
    //             FACE_GRID_HEIGHT as f32,
    //             Color::WHITE,
    //             gizmos,
    //         );
    //     }
    // }
}

fn debug_icoface(icoface: &IcoFace, gizmos: &mut Gizmos) {
    for deltille_slot in icoface.deltille_slots.iter() {
        debug_deltille_slot(deltille_slot, gizmos);
    }
    draw_triangle(
        &icoface.position,
        &icoface.orientation,
        ICOFACE_GRID_WIDTH as f32,
        ICOFACE_GRID_HEIGHT as f32,
        Color::GREEN,
        gizmos,
    )
}

fn debug_deltille_slot(deltille_slot: &DeltilleSlot, gizmos: &mut Gizmos) {
    draw_triangle(
        &deltille_slot.position,
        &deltille_slot.orientation,
        DELTILLE_GRID_WIDTH as f32,
        DELTILLE_GRID_HEIGHT as f32,
        Color::DARK_GREEN,
        gizmos,
    )
    
}

// fn origin_for_icoface_coordinates(x: usize, y: usize) -> Vec2 {
//     let row_origin = ICOFACE_ROW_ORIGINS[y];
//     return Vec2::new(
//         row_origin.x + (x as f32 * (FACE_GRID_WIDTH as f32 + GAP_GRID_WIDTH as f32 * 2.0)),
//         row_origin.y,
//     );
// }

fn draw_triangle(
    origin: &Vec2,
    orientation: &VerticalOrientation,
    width: f32,
    height: f32,
    color: Color,
    gizmos: &mut Gizmos,
) {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    match orientation {
        VerticalOrientation::Up => {
            let left = Vec2::new(origin.x - half_width, origin.y - half_height);
            let right = Vec2::new(origin.x + half_width, origin.y - half_height);
            let top = Vec2::new(origin.x, origin.y + half_height);
            gizmos.linestrip_2d([left, right, top, left], color);
        }
        VerticalOrientation::Down => {
            let left = Vec2::new(origin.x - half_width, origin.y + half_height);
            let right = Vec2::new(origin.x + half_width, origin.y + half_height);
            let down = Vec2::new(origin.x, origin.y - half_height);
            gizmos.linestrip_2d([left, right, down, left], color);
        }
    }
}

// fn offset_for_deltille_component_up_icoface(coordinates: &DeltilleCoordinates) -> Vec2 {
//     let row_size = (coordinates.y + 2) / 2;
//     let x_offset = -(row_size as f32 - 1.0) * DELTILLE_GRID_WIDTH_HALF
//         + (DELTILLE_GRID_WIDTH as f32 * coordinates.x as f32);
//     let visual_row_depth = (coordinates.y + 1) / 2;
//     let y_offset = FACE_GRID_HEIGHT_HALF
//         - DELTILLE_GRID_HEIGHT_HALF
//         - (visual_row_depth as f32 * DELTILLE_GRID_HEIGHT as f32);
//     return Vec2::new(x_offset, y_offset);
// }

// fn offset_for_deltille_component_down_icoface(coordinates: &DeltilleCoordinates) -> Vec2 {
//     let row_size = FACE_DELTILLE_WIDTH - (coordinates.y + 1) / 2;
//     let x_offset = -(row_size as f32 - 1.0) * DELTILLE_GRID_WIDTH_HALF
//         + (DELTILLE_GRID_WIDTH as f32 * coordinates.x as f32);
//     let visual_row_depth = coordinates.y / 2;
//     let y_offset = FACE_GRID_HEIGHT_HALF
//         - DELTILLE_GRID_HEIGHT_HALF
//         - (visual_row_depth as f32 * DELTILLE_GRID_HEIGHT as f32);
//     return Vec2::new(x_offset, y_offset);
// }

// fn draw_deltille_subdivisions_up(
//     origin: &Vec2,
//     color: Color,
//     gizmos: &mut Gizmos,
//     precalculated_coordinates: &Res<PreCalculatedCoordinates>,
// ) {
//     for deltille_coordinates in precalculated_coordinates
//         .all_deltille_coordinates_for_orientation(VerticalOrientation::Up)
//         .iter()
//     {
//         let offset = offset_for_deltille_component_up_icoface(deltille_coordinates);
//         draw_triangle(
//             &Vec2::new(origin.x + offset.x, origin.y + offset.y),
//             if deltille_coordinates.y % 2 == 0 {
//                 &VerticalOrientation::Up
//             } else {
//                 &VerticalOrientation::Down
//             },
//             DELTILLE_GRID_WIDTH as f32,
//             DELTILLE_GRID_HEIGHT as f32,
//             color,
//             gizmos,
//         );
//     }
// }

// fn draw_deltille_subdivisions_down(
//     origin: &Vec2,
//     color: Color,
//     gizmos: &mut Gizmos,
//     precalculated_coordinates: &Res<PreCalculatedCoordinates>,
// ) {
//     for deltille_coordinates in precalculated_coordinates
//         .all_deltille_coordinates_for_orientation(VerticalOrientation::Down)
//         .iter()
//     {
//         let offset = offset_for_deltille_component_down_icoface(deltille_coordinates);
//         draw_triangle(
//             &Vec2::new(origin.x + offset.x, origin.y + offset.y),
//             if deltille_coordinates.y % 2 == 0 {
//                 &VerticalOrientation::Down
//             } else {
//                 &VerticalOrientation::Up
//             },
//             DELTILLE_GRID_WIDTH as f32,
//             DELTILLE_GRID_HEIGHT as f32,
//             color,
//             gizmos,
//         );
//     }
// }

// // // just for fun! replace this with actual sprite choices
// // pub fn place_deltille_sprites(
// //     mut commands: Commands,
// //     icosahedron: Res<Icosahedron>,
// //     precalculated_coordinates: Res<PreCalculatedCoordinates>,
// //     asset_server: Res<AssetServer>,
// // ) {
// //     for (icoface_y, row) in icosahedron.faces.iter().enumerate() {
// //         for (icoface_x, icoface) in row.iter().enumerate() {
// //             let origin = origin_for_icoface_coordinates(icoface_x, icoface_y);
// //             if icoface_y % 2 == 0 {
// //                 place_deltille_sprites_up(
// //                     &mut commands,
// //                     icoface,
// //                     &origin,
// //                     &precalculated_coordinates,
// //                     &asset_server,
// //                 );
// //             } else {
// //                 place_deltille_sprites_down(
// //                     &mut commands,
// //                     icoface,
// //                     &origin,
// //                     &precalculated_coordinates,
// //                     &asset_server,
// //                 );
// //             }
// //         }
// //     }
// // }

// // fn place_deltille_sprites_up(
// //     commands: &mut Commands,
// //     icoface: &IcoFace,
// //     origin: &Vec2,
// //     precalculated_coordinates: &Res<PreCalculatedCoordinates>,
// //     asset_server: &Res<AssetServer>,
// // ) {
// //     for deltille_coordinates in precalculated_coordinates.all_up_deltille_coordinates.iter() {
// //         let offset = offset_for_deltille_component_up_icoface(deltille_coordinates);
// //         let deltille = icoface
// //             .get_deltille_options_at(deltille_coordinates)
// //             .and_then(|options| options.choose(&mut rand::thread_rng()))
// //             .unwrap();
// //         commands.spawn(SpriteBundle {
// //             texture: asset_server.load(&deltille.image_path),
// //             transform: Transform {
// //                 translation: Vec3::new(origin.x + offset.x, origin.y + offset.y, 0.0),
// //                 ..default()
// //             },
// //             sprite: Sprite {
// //                 custom_size: Option::Some(Vec2::new(
// //                     DELTILLE_GRID_WIDTH as f32,
// //                     DELTILLE_GRID_HEIGHT as f32,
// //                 )),
// //                 flip_x: deltille.flip_x,
// //                 flip_y: deltille.flip_y,
// //                 ..default()
// //             },
// //             ..default()
// //         });
// //     }
// // }

// // fn place_deltille_sprites_down(
// //     commands: &mut Commands,
// //     icoface: &IcoFace,
// //     origin: &Vec2,
// //     precalculated_coordinates: &Res<PreCalculatedCoordinates>,
// //     asset_server: &Res<AssetServer>,
// // ) {
// //     for deltille_coordinates in precalculated_coordinates
// //         .all_down_deltille_coordinates
// //         .iter()
// //     {
// //         let offset = offset_for_deltille_component_down_icoface(deltille_coordinates);
// //         let deltille = icoface
// //             .get_deltille_options_at(deltille_coordinates)
// //             .and_then(|options| options.choose(&mut rand::thread_rng()))
// //             .unwrap();
// //         commands.spawn(SpriteBundle {
// //             texture: asset_server.load(&deltille.image_path),
// //             transform: Transform {
// //                 translation: Vec3::new(origin.x + offset.x, origin.y + offset.y, 0.0),
// //                 ..default()
// //             },
// //             sprite: Sprite {
// //                 flip_x: deltille.flip_x,
// //                 flip_y: deltille.flip_y,
// //                 ..default()
// //             },
// //             ..default()
// //         });
// //     }
// // }

// // fn screenshot(main_window: Query<Entity, With<PrimaryWindow>>, mut screenshot_manager: ResMut<ScreenshotManager>) {
// //     fs::create_dir_all("generated").unwrap();
// //     screenshot_manager.save_screenshot_to_disk(main_window.single(), "assets/generated/screenshot.png").unwrap();
// // }
