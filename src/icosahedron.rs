use core::fmt;
use std::array::from_fn;

use crate::config_constants::*;
use bevy::{
    ecs::system::Resource,
    prelude::{Handle, Image},
};

#[derive(Clone, Copy)]
pub enum Orientation {
    /// ∧
    Up,

    /// ∨
    Down,
}

#[derive(Resource)]
pub struct Icosahedron {
    /// ```
    /// ∧   ∧   ∧   ∧   ∧
    /// ∨   ∨   ∨   ∨   ∨
    ///   ∧   ∧   ∧   ∧   ∧
    ///   ∨   ∨   ∨   ∨   ∨
    /// ```
    pub faces: [[IcoFace; 5]; 4],
}

#[derive(Clone)]
pub struct IcoFace {
    pub orientation: Orientation,
    pub deltilles: Vec<Vec<Vec<Deltille>>>,
}

impl IcoFace {
    pub fn with_initial_options(
        orientation: Orientation,
        up_options: &Vec<Deltille>,
        down_options: &Vec<Deltille>,
    ) -> Self {
        return IcoFace {
            orientation,
            deltilles: match orientation {
                Orientation::Up => up_face_deltilles_initial(up_options, down_options),
                Orientation::Down => down_face_deltilles_initial(up_options, down_options),
            },
        };
    }

    pub fn get_deltille_options_at(
        &self,
        coordinates: &DeltilleCoordinates,
    ) -> Option<&Vec<Deltille>> {
        return self
            .deltilles
            .get(coordinates.y)
            .and_then(|row| row.get(coordinates.x));
    }

    pub fn get_mut_deltille_options_at(
        &mut self,
        coordinates: &DeltilleCoordinates,
    ) -> Option<&mut Vec<Deltille>> {
        return self
            .deltilles
            .get_mut(coordinates.y)
            .and_then(|row| row.get_mut(coordinates.x));
    }

}

fn up_face_deltilles_initial(
    up_deltille_options: &Vec<Deltille>,
    down_deltille_options: &Vec<Deltille>,
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
    up_deltille_options: &Vec<Deltille>,
    down_deltille_options: &Vec<Deltille>,
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

#[derive(Clone)]
pub struct Deltille {
    pub image_handle: Handle<Image>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub sockets: Sockets,
}

impl fmt::Debug for Deltille {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.flip_y {"∨"} else {"∧"})
    }
}

#[derive(Clone)]
pub enum Sockets {
    // NW   NE
    //    ∧
    //    S
    Up { nw: String, ne: String, s: String },

    //    N
    //    ∨
    // SW   SE
    Down { n: String, se: String, sw: String },
}

#[derive(Clone, Copy)]
pub struct IcoFaceCoordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct DeltilleCoordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct PreCalculatedCoordinates {
    pub all_ico_face_coordinates: [IcoFaceCoordinates; 20],
    pub all_up_deltille_coordinates: [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT],
    pub all_down_deltille_coordinates: [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT],
}

impl PreCalculatedCoordinates {
    pub fn generate() -> Self {
        return PreCalculatedCoordinates {
            all_ico_face_coordinates: PreCalculatedCoordinates::generate_all_ico_face_coordinates(),
            all_up_deltille_coordinates:
                PreCalculatedCoordinates::generate_all_up_deltille_coordinates(),
            all_down_deltille_coordinates:
                PreCalculatedCoordinates::generate_all_down_deltille_coordinates(),
        };
    }

    fn generate_all_ico_face_coordinates() -> [IcoFaceCoordinates; 20] {
        return from_fn(|i| IcoFaceCoordinates { x: i % 5, y: i / 5 });
    }

    pub fn generate_all_up_deltille_coordinates() -> [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT] {
        let mut coordinates = [DeltilleCoordinates { x: 0, y: 0 }; ICOFACE_DELTILLE_COUNT];
        let mut i = 0;
        let mut rowsize = 1;
        for y in 0..ICOFACE_DELTILLE_ROW_COUNT {
            for x in 0..rowsize {
                coordinates[i] = DeltilleCoordinates { x, y };
                i += 1;
            }
            if y % 2 == 1 {
                rowsize += 1
            };
        }
        return coordinates;
    }

    fn generate_all_down_deltille_coordinates() -> [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT] {
        let mut coordinates = [DeltilleCoordinates { x: 0, y: 0 }; ICOFACE_DELTILLE_COUNT];
        let mut i = 0;
        let mut rowsize = FACE_DELTILLE_WIDTH;
        for y in 0..ICOFACE_DELTILLE_ROW_COUNT {
            for x in 0..rowsize {
                coordinates[i] = DeltilleCoordinates { x, y };
                i += 1;
            }
            if y % 2 == 0 {
                rowsize -= 1
            };
        }
        return coordinates;
    }
}
