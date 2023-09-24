pub const VIEW_SCALE: f32 = 9.0;
pub const DELTILLE_GRID_WIDTH: usize = 16;
pub const GAP_GRID_WIDTH_HALF: usize = 1;
pub const FACE_DELTILLE_WIDTH: usize = 3;

pub const ICOFACE_DELTILLE_COUNT: usize = FACE_DELTILLE_WIDTH.pow(2);
pub const ICOFACE_DELTILLE_ROW_COUNT: usize = 2 * FACE_DELTILLE_WIDTH - 1;
pub const GAP_GRID_WIDTH: usize = GAP_GRID_WIDTH_HALF * 2;
pub const SQRT_0_POINT_75: f32 = 0.86602540378443864676372317075293;
pub const DELTILLE_GRID_HEIGHT: usize = (DELTILLE_GRID_WIDTH as f32 * SQRT_0_POINT_75) as usize + 1;
pub const DELTILLE_GRID_HEIGHT_HALF: f32 = DELTILLE_GRID_HEIGHT as f32 / 2.0;
pub const DELTILLE_GRID_WIDTH_HALF: f32 = DELTILLE_GRID_WIDTH as f32 / 2.0;
pub const FACE_GRID_WIDTH: usize = FACE_DELTILLE_WIDTH as usize * DELTILLE_GRID_WIDTH as usize;
// faces are slightly taller to accomodate imperfect deltille pixel heights
pub const FACE_GRID_HEIGHT: usize = DELTILLE_GRID_HEIGHT as usize * FACE_DELTILLE_WIDTH as usize;
pub const FACE_GRID_HEIGHT_HALF: f32 = FACE_GRID_HEIGHT as f32 * 0.5;
pub const WINDOW_GRID_WIDTH: usize = 5 * (FACE_GRID_WIDTH as usize + GAP_GRID_WIDTH as usize * 2);
pub const WINDOW_GRID_HEIGHT: usize = 2 * FACE_GRID_HEIGHT as usize;