use std::{array::from_fn, fs};

use bevy::prelude::{Res, ResMut, Resource};
use rand::seq::SliceRandom;

use crate::icosahedron::{
    Deltille, FullCoordinates, IcoFace, Icosahedron, Orientation, PreCalculatedCoordinates,
};

#[derive(Resource)]
pub struct WfcState {
    pub steps: Vec<Step>,
    pub complete: bool,
}

fn up_options_from(deltille_inputs: &Vec<Deltille>) -> Vec<Deltille> {
    let mut deltilles = Vec::with_capacity(deltille_inputs.len() * 2);
    for deltille in deltille_inputs.iter() {
        deltilles.push(deltille.clone());
        let mut flipped = deltille.clone();
        flipped.flip_x = true;
        deltilles.push(flipped);
    }
    return deltilles;
}

fn down_options_from(deltille_inputs: &Vec<Deltille>) -> Vec<Deltille> {
    let mut deltilles = Vec::with_capacity(deltille_inputs.len() * 2);
    for deltille in deltille_inputs.iter() {
        let mut clone = deltille.clone();
        clone.flip_y = true;
        deltilles.push(clone);
        let mut flipped = deltille.clone();
        flipped.flip_x = true;
        flipped.flip_y = true;
        deltilles.push(flipped);
    }
    return deltilles;
}

impl WfcState {
    pub fn new() -> Self {
        let deltille_json = fs::read_to_string("assets/deltilles.json").unwrap_or("[]".to_string());
        let deltilles: Vec<Deltille> = serde_json::from_str(&deltille_json).unwrap();

        let up_options = up_options_from(&deltilles);
        let down_options = down_options_from(&deltilles);

        let up_icoface = IcoFace::with_initial_options(Orientation::Up, &up_options, &down_options);
        let down_icoface =
            IcoFace::with_initial_options(Orientation::Down, &up_options, &down_options);

        let icosahedron = Icosahedron {
            faces: [
                from_fn(|_| up_icoface.clone()),
                from_fn(|_| down_icoface.clone()),
                from_fn(|_| up_icoface.clone()),
                from_fn(|_| down_icoface.clone()),
            ],
        };

        WfcState {
            steps: vec![Step { icosahedron }],
            complete: false,
        }
    }
}

pub struct Step {
    icosahedron: Icosahedron,
}

pub fn not_yet_complete(state: Res<WfcState>, coordinates: Res<PreCalculatedCoordinates>) -> bool {
    return !state.complete;
}

pub fn iterate_wfc(
    mut state: ResMut<WfcState>,
    precalculated_coordinates: Res<PreCalculatedCoordinates>,
) {
    let mut icosahedron = &mut state.steps.last_mut().unwrap().icosahedron;
    let deltille_coordinate_options =
        coordinates_with_fewest_possibilities(&icosahedron, &precalculated_coordinates);
    let deltille_coordinate_choice = deltille_coordinate_options.choose(&mut rand::thread_rng());
    match deltille_coordinate_choice {
        Some(chosen_deltille) => {
            match propogate_constraints(icosahedron, chosen_deltille) {
                Ok(_) => println!("remove all options, push this step on the stack and copy a new one"),
                Err(_) => println!("remove option from set and try again"),
            }
        },
        None => println!("discard this step and move back in the stack"),
    };
}

pub fn coordinates_with_fewest_possibilities(
    icosahedron: &Icosahedron,
    coordinates: &Res<PreCalculatedCoordinates>,
) -> Vec<FullCoordinates> {
    let mut fewest_possibilities_so_far = usize::MAX;
    let mut fewest_possibilities_coordinates: Vec<FullCoordinates> = Vec::new();
    for icoface in coordinates.all_ico_face_coordinates.iter() {
        for deltille in coordinates.all_deltille_coordinates_for_icoface(icosahedron.get_icoface(&icoface)) {
            let full_coordinates = FullCoordinates {
                icoface: icoface.clone(),
                deltille: deltille.clone(),
            };
            let len = icosahedron
                .get_deltille_possibilities(&full_coordinates)
                .len();
            if len > 0 && len < fewest_possibilities_so_far {
                fewest_possibilities_so_far = len;
                fewest_possibilities_coordinates.clear();
                fewest_possibilities_coordinates.push(full_coordinates);
            } else if len > 0 && len == fewest_possibilities_so_far {
                fewest_possibilities_coordinates.push(full_coordinates)
            }
        }
    }
    return fewest_possibilities_coordinates;
}

pub fn propogate_constraints(mut icosahedron: &mut Icosahedron, chosen_deltille: &FullCoordinates) -> Result<(),()> {
    return Result::Ok(());
}