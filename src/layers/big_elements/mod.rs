use std::any::TypeId;

use crate::{
    layers::Layer,
    utils::{exclude_layers, pick_random_layer},
};
use big_element_full_circle::BigElementFullCircle;
use big_element_half_circle::BigElementHalfCircle;
use big_element_pill::BigElementPill;
use big_element_pill_ball::BigElementPillBall;
use big_element_pill_split_circle::BigElementPillSplitCircle;
use big_element_quarter_circle::BigElementQuarterCircle;
use big_element_square::BigElementSquare;
use big_element_three_quarter_circle::BigElementThreeQuarterCircle;
use big_element_triangle::BigElementTriangle;
use big_element_two_rectangles::BigElementTwoRectangles;
use big_element_two_squares::BigElementTwoSquares;
use big_element_zig_zag::BigElementZigZag;
use random::Random;

pub mod big_element_full_circle;
pub mod big_element_half_circle;
pub mod big_element_pill;
pub mod big_element_pill_ball;
pub mod big_element_pill_split_circle;
pub mod big_element_quarter_circle;
pub mod big_element_square;
pub mod big_element_three_quarter_circle;
pub mod big_element_triangle;
pub mod big_element_two_rectangles;
pub mod big_element_two_squares;
pub mod big_element_zig_zag;

pub fn random_big_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    // Layers and their weights
    let available_layers: Vec<(Box<dyn Layer>, u32)> = vec![
        (Box::new(BigElementHalfCircle), 100),
        (Box::new(BigElementThreeQuarterCircle), 100),
        (Box::new(BigElementFullCircle), 100),
        (Box::new(BigElementTriangle), 100),
        (Box::new(BigElementTwoSquares), 100),
        (Box::new(BigElementQuarterCircle), 100),
        (Box::new(BigElementZigZag), 50),
        (Box::new(BigElementSquare), 100),
        (Box::new(BigElementPill), 25),
        (Box::new(BigElementPillSplitCircle), 25),
        (Box::new(BigElementTwoRectangles), 50),
        (Box::new(BigElementPillBall), 25),
    ];

    // Filter out the excluded layers
    let allowed_layers = exclude_layers(available_layers, exclusions);

    // Pick a random layer based on the weights of the allowed layers
    pick_random_layer(random, allowed_layers)
}
