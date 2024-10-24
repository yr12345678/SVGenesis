use std::any::TypeId;

use crate::{
    layers::Layer,
    utils::{exclude_layers, pick_random_layer},
};
use overlay_diamond::OverlayDiamond;
use overlay_half_circle::OverlayHalfCircle;
use overlay_triangle::OverlayTriangle;
use random::Random;

pub mod overlay_diamond;
pub mod overlay_half_circle;
pub mod overlay_triangle;

pub fn random_overlay(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(OverlayTriangle), 100),
        (Box::new(OverlayHalfCircle), 100),
        (Box::new(OverlayDiamond), 100),
    ];

    // Filter out the excluded layers
    let allowed_layers = exclude_layers(available_layers, exclusions);

    // Pick a random layer based on the weights of the allowed layers
    pick_random_layer(random, allowed_layers)
}
