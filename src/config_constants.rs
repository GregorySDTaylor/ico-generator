pub const VIEW_SCALE: f32 = 6.0;
pub const DELTILLE_GRID_WIDTH: usize = 16;
pub const ICOFACE_DELTILLE_WIDTH: usize = 4;

pub const ICOFACE_DELTILLE_COUNT: usize = ICOFACE_DELTILLE_WIDTH.pow(2);
pub const ICOSAHEDRON_DELTILLE_COUNT: usize = ICOFACE_DELTILLE_COUNT * 20;
pub const SQRT_0_POINT_75: f32 = 0.86602540378443864676372317075293;

// accomodate imperfect deltille pixel heights
pub const DELTILLE_GRID_HEIGHT: usize = (DELTILLE_GRID_WIDTH as f32 * SQRT_0_POINT_75) as usize + 1;
pub const DELTILLE_GRID_HEIGHT_HALF: f32 = DELTILLE_GRID_HEIGHT as f32 / 2.0;
pub const ICOFACE_GRID_WIDTH: usize =
    ICOFACE_DELTILLE_WIDTH as usize * DELTILLE_GRID_WIDTH as usize;
pub const ICOFACE_GRID_WIDTH_HALF: usize = ICOFACE_GRID_WIDTH / 2;

// faces are slightly taller to accomodate imperfect deltille pixel heights
pub const ICOFACE_GRID_HEIGHT: usize =
    DELTILLE_GRID_HEIGHT as usize * ICOFACE_DELTILLE_WIDTH as usize;
pub const ICOFACE_GRID_HEIGHT_HALF: f32 = ICOFACE_GRID_HEIGHT as f32 * 0.5;
pub const WINDOW_GRID_WIDTH: usize = 5 * ICOFACE_GRID_WIDTH + ICOFACE_GRID_WIDTH_HALF;
pub const WINDOW_GRID_HEIGHT: usize = 3 * ICOFACE_GRID_HEIGHT as usize;
