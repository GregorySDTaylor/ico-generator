use bevy::prelude::*;

const VIEW_SCALE: f32 = 20.;
const SIDE_LENGTH_HALF: u16 = 8;
const SIDE_LENGTH: u16 = SIDE_LENGTH_HALF * 2;
const GAP_SIZE_HALF: u16 = 1;
const GAP_SIZE: u16 = GAP_SIZE_HALF * 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Icosphere Texture Generator".into(),
                resolution: (window_width_px(), window_height_px()).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_triangles)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}

fn draw_triangles(mut gizmos: Gizmos) {
    let width_pixels: f32 = window_width_px();
    let x_offset = -0.5 * width_pixels;
    let height_pixels: f32 = window_height_px();
    let y_offset = -0.5 * height_pixels;
    let x_max: f32 = width_pixels / 2.;
    let y_max: f32 = height_pixels / 2.;
    let x_min: f32 = -x_max;
    let y_min: f32 = -y_max;
    let grid_count_x = grid_count_x();
    let grid_count_y = grid_count_y();

    // draw grid
    for i in 0..grid_count_x {
        let x = i as f32 * VIEW_SCALE + x_offset;
        gizmos.line_2d(Vec2::new(x, y_min), Vec2::new(x, y_max), Color::DARK_GRAY);
    }

    for i in 0..grid_count_y {
        let y = i as f32 * VIEW_SCALE + y_offset;
        gizmos.line_2d(Vec2::new(x_min, y), Vec2::new(x_max, y), Color::DARK_GRAY);
    }

    // bottom row triangles
    let period_x = (SIDE_LENGTH + GAP_SIZE) as f32 * VIEW_SCALE;
    let half_side_left_min_x = x_min - SIDE_LENGTH_HALF as f32 * VIEW_SCALE;
    let mut origin = Vec2::new(half_side_left_min_x, y_min);
    for _ in 0..6 {
        triangle_up(origin, &mut gizmos);
        origin = Vec2::new(origin.x + period_x, origin.y);
    }

    // row 2 triangles
    let half_gap_right_min_x = x_min + GAP_SIZE_HALF as f32 * VIEW_SCALE;
    origin = Vec2::new(half_gap_right_min_x, 0.);
    for _ in 0..5 {
        triangle_down(origin, &mut gizmos);
        origin = Vec2::new(origin.x + period_x, origin.y);
    }

    // row 3 triangles
    origin = Vec2::new(half_gap_right_min_x, 0.);
    for _ in 0..5 {
        triangle_up(origin, &mut gizmos);
        origin = Vec2::new(origin.x + period_x, origin.y);
    }

    // top row triangles
    origin = Vec2::new(half_side_left_min_x, y_max);
    for _ in 0..6 {
        triangle_down(origin, &mut gizmos);
        origin = Vec2::new(origin.x + period_x, origin.y);
    }

}

fn grid_count_x() -> u16 {
    return (SIDE_LENGTH  + GAP_SIZE) * 5;
}

fn triangle_grid_height() -> u16 {
    return (SIDE_LENGTH as f32 * f32::sqrt(0.75)).ceil() as u16;
}

fn grid_count_y() -> u16 {
    return triangle_grid_height() * 2;
}

fn window_width_px() -> f32 {
    return grid_count_x() as f32 * VIEW_SCALE;
}

fn window_height_px() -> f32 {
    return grid_count_y() as f32 * VIEW_SCALE;
}

fn triangle_up(origin: Vec2, gizmos: &mut Gizmos) {
    let right_x = origin.x + SIDE_LENGTH as f32 * VIEW_SCALE;
    let right = Vec2::new(right_x, origin.y);
    let top_x = origin.x + SIDE_LENGTH as f32 / 2. * VIEW_SCALE;
    let top_y = origin.y + SIDE_LENGTH as f32 * f32::sqrt(0.75) * VIEW_SCALE;
    let top = Vec2::new(top_x, top_y);
    let color = Color::GREEN;
    gizmos.line_2d(origin, right, color);
    gizmos.line_2d(origin, top, color);
    gizmos.line_2d(top, right, color);
}

fn triangle_down(origin: Vec2, gizmos: &mut Gizmos) {
    let right_x = origin.x + SIDE_LENGTH as f32 * VIEW_SCALE;
    let right = Vec2::new(right_x, origin.y);
    let top_x = origin.x + SIDE_LENGTH as f32 / 2. * VIEW_SCALE;
    let top_y = origin.y - SIDE_LENGTH as f32 * f32::sqrt(0.75) * VIEW_SCALE;
    let top = Vec2::new(top_x, top_y);
    let color = Color::GREEN;
    gizmos.line_2d(origin, right, color);
    gizmos.line_2d(origin, top, color);
    gizmos.line_2d(top, right, color);
}