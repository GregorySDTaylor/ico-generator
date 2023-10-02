use core::fmt;
use serde::{Deserialize, Serialize};
use std::array::from_fn;

use crate::config_constants::*;
use bevy::ecs::system::Resource;

#[derive(Clone, Copy)]
pub enum Orientation {
    /// ∧
    Up,

    /// ∨
    Down,
}

// TODO: maybe the icofaces and detilles just need an id, not a coordinate
// TODO: Probably want an adjacency list

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

impl Icosahedron {
    pub fn get_icoface(&self, coordinates: &IcoFaceCoordinates) -> &IcoFace {
        return &self.faces[coordinates.y][coordinates.x];
    }

    pub fn get_deltille_possibilities(&self, coordinates: &FullCoordinates) -> &Vec<Deltille> {
        return &self.faces[coordinates.icoface.y][coordinates.icoface.x]
            .deltilles
            .get(coordinates.deltille.y)
            .and_then(|row| row.get(coordinates.deltille.x))
            .unwrap();
    }
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

fn default_flip() -> bool {
    false
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Deltille {
    pub image_path: String,
    #[serde(default = "default_flip")]
    pub flip_x: bool,
    #[serde(default = "default_flip")]
    pub flip_y: bool,
    pub sockets: Sockets,
}

impl fmt::Debug for Deltille {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.flip_y { "∨" } else { "∧" })
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug)]
pub struct IcoFaceCoordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct DeltilleCoordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct FullCoordinates {
    pub icoface: IcoFaceCoordinates,
    pub deltille: DeltilleCoordinates,
}

#[derive(Resource)]
pub struct PreCalculatedCoordinates {
    pub all_ico_face_coordinates: [IcoFaceCoordinates; 20],
    all_up_deltille_coordinates: [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT],
    all_down_deltille_coordinates: [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT],
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

    pub fn all_deltille_coordinates_for_orientation(
        &self,
        orientation: Orientation,
    ) -> [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT] {
        return match orientation {
            Orientation::Up => self.all_up_deltille_coordinates,
            Orientation::Down => self.all_down_deltille_coordinates,
        };
    }

    pub fn all_deltille_coordinates_for_icoface(
        &self,
        icoface: &IcoFace,
    ) -> [DeltilleCoordinates; ICOFACE_DELTILLE_COUNT] {
        return self.all_deltille_coordinates_for_orientation(icoface.orientation);
    }
}
