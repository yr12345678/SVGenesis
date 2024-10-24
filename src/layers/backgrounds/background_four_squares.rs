use std::any::Any;

use crate::hsl::*;
use crate::layers::big_elements;
use crate::layers::Layer;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct BackgroundFourSquares;

impl Layer for BackgroundFourSquares {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the squares that will make up the four squares background
        let mut rectangle1 = Rectangle::new() // Top-left
            .set("x", 0)
            .set("y", 0)
            .set("width", "50%")
            .set("height", "50%");

        let mut rectangle2 = Rectangle::new() // Top-right
            .set("x", 500)
            .set("y", 0)
            .set("width", "50%")
            .set("height", "50%");

        let mut rectangle3 = Rectangle::new() // Bottom-left
            .set("x", 0)
            .set("y", 500)
            .set("width", "50%")
            .set("height", "50%");

        let mut rectangle4 = Rectangle::new() // Bottom-right
            .set("x", 500)
            .set("y", 500)
            .set("width", "50%")
            .set("height", "50%");

        // Pick random solid colors. No gradients here, too messy.
        let (color1, color2, color3, color4) = if base_color.is_some() {
            // Use the base color
            (
                base_color.unwrap().derive_similar_color(random).as_string(),
                base_color.unwrap().derive_similar_color(random).as_string(),
                base_color.unwrap().derive_similar_color(random).as_string(),
                base_color.unwrap().derive_similar_color(random).as_string(),
            )
        } else {
            // Pick a random color
            let roll = random.roll::<u8>(100);
            let color_mode = if roll < 20 {
                ColorMode::Tone
            } else if roll < 50 {
                ColorMode::Light
            } else {
                ColorMode::Vibrant
            };

            (
                HSL::new_random(random, color_mode, 100).as_string(),
                HSL::new_random(random, color_mode, 100).as_string(),
                HSL::new_random(random, color_mode, 100).as_string(),
                HSL::new_random(random, color_mode, 100).as_string(),
            )
        };

        // Add the fill to the rectangles
        rectangle1 = rectangle1.set("fill", color1);
        rectangle2 = rectangle2.set("fill", color2);
        rectangle3 = rectangle3.set("fill", color3);
        rectangle4 = rectangle4.set("fill", color4);

        vec![
            rectangle1.into(),
            rectangle2.into(),
            rectangle3.into(),
            rectangle4.into(),
        ]
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::big_element_two_squares::BigElementTwoSquares.type_id(), // The two squares big element doesn't differentiate from this background
        ]
    }
}
