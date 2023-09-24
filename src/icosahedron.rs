use bevy::{ecs::component::Component, prelude::{Handle, Image}};

#[derive(Component)]
pub struct Icosahedron {

    // ∧   ∧   ∧   ∧   ∧
    // ∨   ∨   ∨   ∨   ∨
    //   ∧   ∧   ∧   ∧   ∧
    //   ∨   ∨   ∨   ∨   ∨
    pub faces: [[IcoFace; 5]; 4],

}

#[derive(Clone)]
pub struct IcoFace {
    pub deltilles: Vec<Vec<Vec<Deltille>>>
}

#[derive(Clone)]
pub struct Deltille {
    pub image_handle: Handle<Image>,
    pub flip_x: bool,
    pub sockets: Sockets
}

#[derive(Clone)]
pub enum Sockets {

    // NW   NE
    //    ∧
    //    S
    Up {
        nw: String,
        ne: String,
        s: String,
    },

    //    N
    //    ∨
    // SW   SE
    Down {
        n: String,
        se: String,
        sw: String,
    }

}
