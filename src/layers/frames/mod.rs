use std::any::TypeId;

use crate::{
    layers::Layer,
    utils::{exclude_layers, pick_random_layer},
};
use frame_straight::FrameStraight;
use random::Random;

pub mod frame_straight;

pub fn random_frame(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![(Box::new(FrameStraight), 100)];

    // Filter out the excluded layers
    let allowed_layers = exclude_layers(available_layers, exclusions);

    // Pick a random layer based on the weights of the allowed layers
    pick_random_layer(random, allowed_layers)
}
