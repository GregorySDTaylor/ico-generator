use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::config_constants::*;
use bevy::prelude::{Vec2, Resource};

// TODO: validate build deltilles function
// TODO: array[boolean]-backed hash set?
// TODO: just use hashmaps?
// TODO: use Rc for tile option references?
// TODO: flat diltille array and adjacency list?

trait ArrayIndex {
    fn index(&self) -> usize;
}

/// ```
/// Up | Down
/// ∧  |  ∨
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalOrientation {
    /// ∧
    Up,

    /// ∨
    Down,
}

impl ArrayIndex for VerticalOrientation {
    fn index(&self) -> usize {
        match self {
            VerticalOrientation::Up => 0,
            VerticalOrientation::Down => 1,
        }
    }
}

const VERTICAL_ORIENTATION_COUNT: usize = 2;

/// ```
/// NW   NE |    N
///    ∧    |    ∨
///    S    | SW   SE
/// ```
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeltilleFaceSocket {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl ArrayIndex for DeltilleFaceSocket {
    fn index(&self) -> usize {
        match self {
            DeltilleFaceSocket::N => 0,
            DeltilleFaceSocket::NE => 0,
            DeltilleFaceSocket::SE => 1,
            DeltilleFaceSocket::S => 1,
            DeltilleFaceSocket::SW => 2,
            DeltilleFaceSocket::NW => 2,
        }
    }
}

const SOCKET_COUNT: usize = 3;

#[derive(Resource)]
pub struct Icosahedron {
    /// ```
    /// ∧   ∧   ∧   ∧   ∧
    /// ∨   ∨   ∨   ∨   ∨
    ///   ∧   ∧   ∧   ∧   ∧
    ///   ∨   ∨   ∨   ∨   ∨
    /// ```
    pub icofaces: [IcoFace; 20],
}

impl Icosahedron {
    pub fn new(options: &[HashSet<usize>; VERTICAL_ORIENTATION_COUNT]) -> Self {
        let mut icofaces_in_progress: Vec<IcoFace> = Vec::with_capacity(20);
        let mut icoface_index = 0;

        // top row
        let mut position = Vec2 {
            x: 0.5 * ICOFACE_GRID_WIDTH as f32,
            y: 2.5 * ICOFACE_GRID_HEIGHT as f32,
        };
        for _ in 0..5 {
            icofaces_in_progress.push(IcoFace::new(
                VerticalOrientation::Up,
                position,
                options,
                [
                    // NE
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::NW,
                        target_icoface_id: (icoface_index + 1) % 5,
                    },
                    // S
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::N,
                        target_icoface_id: icoface_index + 5,
                    },
                    // NW
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::NE,
                        // +4 is like -1 but avoids a subtract overflow in the usize
                        target_icoface_id: (icoface_index + 4) % 5,
                    },
                ],
                0,
            ));
            icoface_index += 1;
            position.x += ICOFACE_GRID_WIDTH as f32;
        }

        // second row "down" icofaces
        position.x = 0.5 * ICOFACE_GRID_WIDTH as f32;
        position.y = 1.5 * ICOFACE_GRID_HEIGHT as f32;
        for _ in 0..5 {
            icofaces_in_progress.push(IcoFace::new(
                VerticalOrientation::Down,
                position,
                options,
                [
                    // N
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::S,
                        target_icoface_id: icoface_index - 5,
                    },
                    // SE
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::NW,
                        target_icoface_id: 10 + (icoface_index % 5),
                    },
                    // SW
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::NE,
                        target_icoface_id: 10 + ((icoface_index - 1) % 5),
                    },
                ],
                0,
            ));
            icoface_index += 1;
            position.x += ICOFACE_GRID_WIDTH as f32;
        }

        // second row "up" icofaces
        position.x = ICOFACE_GRID_WIDTH as f32;
        for _ in 0..5 {
            icofaces_in_progress.push(IcoFace::new(
                VerticalOrientation::Up,
                position,
                options,
                [
                    // NE
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::SW,
                        target_icoface_id: 5 + ((icoface_index + 1) % 5),
                    },
                    // S
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::N,
                        target_icoface_id: icoface_index + 5,
                    },
                    // NW
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::SE,
                        target_icoface_id: 5 + (icoface_index % 5),
                    },
                ],
                0,
            ));
            icoface_index += 1;
            position.x += ICOFACE_GRID_WIDTH as f32;
        }

        // bottom row "down" icofaces
        position.x = ICOFACE_GRID_WIDTH as f32;
        position.y = 0.5 * ICOFACE_GRID_HEIGHT as f32;
        for _ in 0..5 {
            icofaces_in_progress.push(IcoFace::new(
                VerticalOrientation::Down,
                position,
                options,
                [
                    // N
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::S,
                        target_icoface_id: icoface_index - 5,
                    },
                    // SE
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::SW,
                        target_icoface_id: 15 + ((icoface_index + 1) % 5),
                    },
                    // SW
                    IcoFaceConnection {
                        target_socket: DeltilleFaceSocket::SE,
                        target_icoface_id: 15 + ((icoface_index - 1) % 5),
                    },
                ],
                0,
            ));
            icoface_index += 1;
            position.x += ICOFACE_GRID_WIDTH as f32;
        }

        let icofaces = icofaces_in_progress.try_into().unwrap();
        return Icosahedron { icofaces };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IcoFaceConnection {
    pub target_socket: DeltilleFaceSocket,
    pub target_icoface_id: usize,
}

#[derive(Clone, Debug)]
pub struct IcoFace {
    pub orientation: VerticalOrientation,
    pub position: Vec2,
    pub deltille_slots: [DeltilleSlot; ICOFACE_DELTILLE_COUNT],
    pub icoface_connections: [IcoFaceConnection; SOCKET_COUNT],
}

#[derive(Debug, Copy, Clone)]
pub struct DeltilleConnection {
    pub target_socket: DeltilleFaceSocket,
    pub target_deltille_coordinates: DeltilleSlotId,
}

impl IcoFace {
    pub fn new(
        vertical_orientation: VerticalOrientation,
        position: Vec2,
        options: &[HashSet<usize>; VERTICAL_ORIENTATION_COUNT],
        icoface_connections: [IcoFaceConnection; SOCKET_COUNT],
        this_icoface_index: usize,
    ) -> Self {
        let mut deltille_slots = Self::generate_deltille_slots(
            position,
            vertical_orientation,
            options,
            icoface_connections,
            this_icoface_index,
        );
        return IcoFace {
            orientation: vertical_orientation,
            position,
            deltille_slots,
            icoface_connections,
        };
    }

    fn generate_deltille_slots(
        icoface_position: Vec2,
        vertical_orientation: VerticalOrientation,
        options: &[HashSet<usize>; VERTICAL_ORIENTATION_COUNT],
        icoface_connections: [IcoFaceConnection; SOCKET_COUNT],
        this_icoface_index: usize,
    ) -> [DeltilleSlot; ICOFACE_DELTILLE_COUNT] {
        let mut deltille_slots_in_progress: Vec<DeltilleSlot> =
            Vec::with_capacity(ICOFACE_DELTILLE_COUNT);
        let mut row: usize = 0;
        // let mut position_in_row: usize = 0;
        // let mut orientation = vertical_orientation;
        let mut deltille_position = icoface_position.clone();
        deltille_position.y += ICOFACE_GRID_HEIGHT_HALF - DELTILLE_GRID_HEIGHT_HALF;
        let mut row_size: usize;
        let mut deltille_index = 0;

        match vertical_orientation {
            // build deltilles for "up" icoface
            VerticalOrientation::Up => {
                row_size = 0;
                while row < ICOFACE_DELTILLE_WIDTH {
                    deltille_position.x = Self::row_reset_position_x(icoface_position.x, row_size);

                    // build inside "down" deltille slots
                    for _ in 0..row_size {
                        let orientation = VerticalOrientation::Down;
                        let deltille_option_ids = options[orientation.index()].clone();
                        let connections: [DeltilleConnection; SOCKET_COUNT] = [
                            // N
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::S,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index - row_size,
                                },
                            },
                            // SE
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::NW,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size + 1,
                                },
                            },
                            // SW
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::NE,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size,
                                },
                            },
                        ];
                        deltille_slots_in_progress.push(DeltilleSlot {
                            deltille_option_ids,
                            position: deltille_position.clone(),
                            orientation,
                            connections,
                        });
                        deltille_position.x += DELTILLE_GRID_WIDTH as f32;
                        deltille_index += 1;
                    }

                    row_size += 1;
                    deltille_position.x = Self::row_reset_position_x(icoface_position.x, row_size);

                    // build outside "up" deltille slots
                    for i in 0..row_size {
                        let orientation = VerticalOrientation::Up;
                        let deltille_option_ids = options[orientation.index()].clone();

                        let connection_ne: DeltilleConnection = if i == row_size - 1 {
                            // if exposed to icoface edge, connect to adjacent icoface
                            let target_socket =
                                icoface_connections[DeltilleFaceSocket::NE.index()].target_socket;
                            let target_icoface_id = icoface_connections
                                [DeltilleFaceSocket::NE.index()]
                            .target_icoface_id;
                            let target_deltille_id = Self::exposed_deltille_id(row, target_socket);
                            DeltilleConnection {
                                target_socket,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: target_icoface_id,
                                    deltille_id: target_deltille_id,
                                },
                            }
                        } else {
                            // connect to a deltille in the same icoface
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::SW,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index - row_size + 1,
                                },
                            }
                        };

                        let connection_s: DeltilleConnection = if row == ICOFACE_DELTILLE_WIDTH - 1
                        {
                            // if exposed to icoface edge, connect to adjacent icoface
                            let target_socket =
                                icoface_connections[DeltilleFaceSocket::S.index()].target_socket;
                            let target_icoface_id = icoface_connections
                                [DeltilleFaceSocket::S.index()]
                            .target_icoface_id;
                            let target_deltille_id = Self::exposed_deltille_id(i, target_socket);
                            DeltilleConnection {
                                target_socket,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: target_icoface_id,
                                    deltille_id: target_deltille_id,
                                },
                            }
                        } else {
                            // connect to a deltille in the same icoface
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::N,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size,
                                },
                            }
                        };

                        let connection_nw: DeltilleConnection = if i == 0 {
                            // if exposed to icoface edge, connect to adjacent icoface
                            let target_socket =
                                icoface_connections[DeltilleFaceSocket::NW.index()].target_socket;
                            let target_icoface_id = icoface_connections
                                [DeltilleFaceSocket::NW.index()]
                            .target_icoface_id;
                            let target_deltille_id = Self::exposed_deltille_id(row, target_socket);
                            DeltilleConnection {
                                target_socket,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: target_icoface_id,
                                    deltille_id: target_deltille_id,
                                },
                            }
                        } else {
                            // connect to a deltille in the same icoface
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::SE,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index - row_size,
                                },
                            }
                        };

                        deltille_slots_in_progress.push(DeltilleSlot {
                            deltille_option_ids,
                            position: deltille_position.clone(),
                            orientation,
                            connections: [connection_ne, connection_s, connection_nw],
                        });
                        deltille_position.x += DELTILLE_GRID_WIDTH as f32;
                        deltille_index += 1;
                    }

                    deltille_position.y = deltille_position.y - DELTILLE_GRID_HEIGHT as f32;
                    row += 1;
                }
            }

            // build deltilles for "down" icoface
            VerticalOrientation::Down => {
                row_size = ICOFACE_DELTILLE_WIDTH;
                while row < ICOFACE_DELTILLE_WIDTH {
                    deltille_position.x = Self::row_reset_position_x(icoface_position.x, row_size);

                    // build outside "down" deltille slots
                    for i in 0..row_size {
                        let orientation = VerticalOrientation::Down;
                        let deltille_option_ids = options[orientation.index()].clone();

                        let connection_n: DeltilleConnection = if row == 0 {
                            // if exposed to icoface edge, connect to adjacent icoface
                            let target_socket =
                                icoface_connections[DeltilleFaceSocket::N.index()].target_socket;
                            let target_icoface_id = icoface_connections
                                [DeltilleFaceSocket::N.index()]
                            .target_icoface_id;
                            let target_deltille_id = Self::exposed_deltille_id(i, target_socket);
                            DeltilleConnection {
                                target_socket,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: target_icoface_id,
                                    deltille_id: target_deltille_id,
                                },
                            }
                        } else {
                            // connect to a deltille in the same icoface
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::S,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index - row_size,
                                },
                            }
                        };

                        let connection_se: DeltilleConnection = if i == row_size - i {
                            // if exposed to icoface edge, connect to adjacent icoface
                            let target_socket =
                                icoface_connections[DeltilleFaceSocket::SE.index()].target_socket;
                            let target_icoface_id = icoface_connections
                                [DeltilleFaceSocket::SE.index()]
                            .target_icoface_id;
                            let target_deltille_id = Self::exposed_deltille_id(row, target_socket);
                            DeltilleConnection {
                                target_socket,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: target_icoface_id,
                                    deltille_id: target_deltille_id,
                                },
                            }
                        } else {
                            // connect to a deltille in the same icoface
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::NW,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size,
                                },
                            }
                        };

                        let connection_sw: DeltilleConnection = if i == 0 {
                            // if exposed to icoface edge, connect to adjacent icoface
                            let target_socket =
                                icoface_connections[DeltilleFaceSocket::SW.index()].target_socket;
                            let target_icoface_id = icoface_connections
                                [DeltilleFaceSocket::SW.index()]
                            .target_icoface_id;
                            let target_deltille_id = Self::exposed_deltille_id(row, target_socket);
                            DeltilleConnection {
                                target_socket,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: target_icoface_id,
                                    deltille_id: target_deltille_id,
                                },
                            }
                        } else {
                            // connect to a deltille in the same icoface
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::NE,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size - 1,
                                },
                            }
                        };

                        deltille_slots_in_progress.push(DeltilleSlot {
                            deltille_option_ids,
                            position: deltille_position.clone(),
                            orientation,
                            connections: [connection_n, connection_se, connection_sw],
                        });
                        deltille_position.x += DELTILLE_GRID_WIDTH as f32;
                        deltille_index += 1;
                    }

                    row_size -= 1;
                    deltille_position.x = Self::row_reset_position_x(icoface_position.x, row_size);

                    // build inside "up" deltille slots
                    for _ in 0..row_size {
                        let orientation = VerticalOrientation::Up;
                        let deltille_option_ids = options[orientation.index()].clone();
                        let connections: [DeltilleConnection; SOCKET_COUNT] = [
                            // NE
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::SW,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index - row_size,
                                },
                            },
                            // S
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::N,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size,
                                },
                            },
                            // NW
                            DeltilleConnection {
                                target_socket: DeltilleFaceSocket::SE,
                                target_deltille_coordinates: DeltilleSlotId {
                                    icoface_id: this_icoface_index,
                                    deltille_id: deltille_index + row_size,
                                },
                            },
                        ];
                        deltille_slots_in_progress.push(DeltilleSlot {
                            deltille_option_ids,
                            position: deltille_position.clone(),
                            orientation,
                            connections,
                        });
                        deltille_position.x += DELTILLE_GRID_WIDTH as f32;
                        deltille_index += 1;
                    }

                    deltille_position.y = deltille_position.y - DELTILLE_GRID_HEIGHT as f32;
                    row += 1;
                }
            }
        };
        return deltille_slots_in_progress.try_into().unwrap();
    }

    fn row_reset_position_x(icoface_origin_x: f32, row_size: usize) -> f32 {
        icoface_origin_x - ((row_size as f32 - 1.) / 2. * DELTILLE_GRID_WIDTH as f32)
    }

    /// Returns an index for an exposed deltille within an icoface given
    ///
    /// # Arguments
    ///
    /// * `position` - the 0-based index of the detille with respect to its icoface edge,
    /// top to bottom and left to right
    ///
    /// * `orientation` - the orientation of the icoface edge
    fn exposed_deltille_id(position: usize, orientation: DeltilleFaceSocket) -> usize {
        return match orientation {
            DeltilleFaceSocket::N => position,
            DeltilleFaceSocket::NE => (position + 1).pow(2) - 1,
            DeltilleFaceSocket::SE => {
                ICOFACE_DELTILLE_COUNT
                    - 1
                    - (ICOFACE_DELTILLE_WIDTH - position - 1) * (ICOFACE_DELTILLE_WIDTH - position)
            }
            DeltilleFaceSocket::S => ICOFACE_DELTILLE_COUNT - (ICOFACE_DELTILLE_WIDTH - position),
            DeltilleFaceSocket::SW => {
                ICOFACE_DELTILLE_COUNT - (ICOFACE_DELTILLE_WIDTH - position).pow(2)
            }
            DeltilleFaceSocket::NW => position * (position + 1),
        };
    }
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
    pub sockets: [String; SOCKET_COUNT],
}

#[derive(Clone, Copy, Debug)]
pub struct DeltilleSlotId {
    pub icoface_id: usize,
    pub deltille_id: usize,
}

#[derive(Clone, Debug)]
pub struct DeltilleSlot {
    pub deltille_option_ids: HashSet<usize>,
    pub position: Vec2,
    pub orientation: VerticalOrientation,
    pub connections: [DeltilleConnection; SOCKET_COUNT],
}

#[cfg(test)]
mod tests {
    use std::{array::from_fn, collections::HashSet};

    use bevy::prelude::Vec2;

    use crate::{
        config_constants::{ICOFACE_DELTILLE_COUNT, ICOFACE_DELTILLE_WIDTH},
        icosahedron::{
            self, DeltilleFaceSocket, IcoFace, IcoFaceConnection, Icosahedron, VerticalOrientation,
        },
    };

    #[test]
    fn generate_deltille_slots_works() {
        let position = Vec2 { x: 0., y: 0. };
        let result_up = IcoFace::generate_deltille_slots(
            position,
            VerticalOrientation::Up,
            &from_fn(|_| HashSet::new()),
            [
                IcoFaceConnection {
                    target_socket: DeltilleFaceSocket::N,
                    target_icoface_id: 0,
                },
                IcoFaceConnection {
                    target_socket: DeltilleFaceSocket::N,
                    target_icoface_id: 0,
                },
                IcoFaceConnection {
                    target_socket: DeltilleFaceSocket::N,
                    target_icoface_id: 0,
                },
            ],
            0,
        );
        assert_eq!(result_up.len(), ICOFACE_DELTILLE_COUNT);

        let result_down = IcoFace::generate_deltille_slots(
            position,
            VerticalOrientation::Down,
            &from_fn(|_| HashSet::new()),
            [
                IcoFaceConnection {
                    target_socket: DeltilleFaceSocket::N,
                    target_icoface_id: 0,
                },
                IcoFaceConnection {
                    target_socket: DeltilleFaceSocket::N,
                    target_icoface_id: 0,
                },
                IcoFaceConnection {
                    target_socket: DeltilleFaceSocket::N,
                    target_icoface_id: 0,
                },
            ],
            0,
        );
        assert_eq!(result_down.len(), ICOFACE_DELTILLE_COUNT);
    }

    #[test]
    fn exposed_deltille_index_n() {
        assert_eq!(IcoFace::exposed_deltille_id(0, DeltilleFaceSocket::N), 0);
        if ICOFACE_DELTILLE_WIDTH >= 2 {
            assert_eq!(IcoFace::exposed_deltille_id(1, DeltilleFaceSocket::N), 1);
        }
        if ICOFACE_DELTILLE_WIDTH >= 3 {
            assert_eq!(IcoFace::exposed_deltille_id(2, DeltilleFaceSocket::N), 2);
        }
        if ICOFACE_DELTILLE_WIDTH >= 4 {
            assert_eq!(IcoFace::exposed_deltille_id(3, DeltilleFaceSocket::N), 3);
        }
    }

    #[test]
    fn exposed_deltille_index_ne() {
        assert_eq!(IcoFace::exposed_deltille_id(0, DeltilleFaceSocket::NE), 0);
        if ICOFACE_DELTILLE_WIDTH >= 2 {
            assert_eq!(IcoFace::exposed_deltille_id(1, DeltilleFaceSocket::NE), 3);
        }
        if ICOFACE_DELTILLE_WIDTH >= 3 {
            assert_eq!(IcoFace::exposed_deltille_id(2, DeltilleFaceSocket::NE), 8);
        }
        if ICOFACE_DELTILLE_WIDTH >= 4 {
            assert_eq!(IcoFace::exposed_deltille_id(3, DeltilleFaceSocket::NE), 15);
        }
    }

    #[test]
    fn exposed_deltille_index_se() {
        assert_eq!(IcoFace::exposed_deltille_id(0, DeltilleFaceSocket::SE), 3);
        if ICOFACE_DELTILLE_WIDTH >= 2 {
            assert_eq!(IcoFace::exposed_deltille_id(1, DeltilleFaceSocket::SE), 9);
        }
        if ICOFACE_DELTILLE_WIDTH >= 3 {
            assert_eq!(IcoFace::exposed_deltille_id(2, DeltilleFaceSocket::SE), 13);
        }
        if ICOFACE_DELTILLE_WIDTH >= 4 {
            assert_eq!(IcoFace::exposed_deltille_id(3, DeltilleFaceSocket::SE), 15);
        }
    }

    #[test]
    fn exposed_deltille_index_s() {
        assert_eq!(IcoFace::exposed_deltille_id(0, DeltilleFaceSocket::S), 12);
        if ICOFACE_DELTILLE_WIDTH >= 2 {
            assert_eq!(IcoFace::exposed_deltille_id(1, DeltilleFaceSocket::S), 13);
        }
        if ICOFACE_DELTILLE_WIDTH >= 3 {
            assert_eq!(IcoFace::exposed_deltille_id(2, DeltilleFaceSocket::S), 14);
        }
        if ICOFACE_DELTILLE_WIDTH >= 4 {
            assert_eq!(IcoFace::exposed_deltille_id(3, DeltilleFaceSocket::S), 15);
        }
    }

    #[test]
    fn exposed_deltille_index_sw() {
        assert_eq!(IcoFace::exposed_deltille_id(0, DeltilleFaceSocket::SW), 0);
        if ICOFACE_DELTILLE_WIDTH >= 2 {
            assert_eq!(IcoFace::exposed_deltille_id(1, DeltilleFaceSocket::SW), 7);
        }
        if ICOFACE_DELTILLE_WIDTH >= 3 {
            assert_eq!(IcoFace::exposed_deltille_id(2, DeltilleFaceSocket::SW), 12);
        }
        if ICOFACE_DELTILLE_WIDTH >= 4 {
            assert_eq!(IcoFace::exposed_deltille_id(3, DeltilleFaceSocket::SW), 15);
        }
    }

    #[test]
    fn exposed_deltille_index_nw() {
        assert_eq!(IcoFace::exposed_deltille_id(0, DeltilleFaceSocket::NW), 0);
        if ICOFACE_DELTILLE_WIDTH >= 2 {
            assert_eq!(IcoFace::exposed_deltille_id(1, DeltilleFaceSocket::NW), 2);
        }
        if ICOFACE_DELTILLE_WIDTH >= 3 {
            assert_eq!(IcoFace::exposed_deltille_id(2, DeltilleFaceSocket::NW), 6);
        }
        if ICOFACE_DELTILLE_WIDTH >= 4 {
            assert_eq!(IcoFace::exposed_deltille_id(3, DeltilleFaceSocket::NW), 12);
        }
    }

    #[test]
    fn new_icosahedron_works() {
        let icosahedron = Icosahedron::new(&from_fn(|_| HashSet::new()));
        assert_eq!(icosahedron.icofaces.len(), 20);
    }

}
