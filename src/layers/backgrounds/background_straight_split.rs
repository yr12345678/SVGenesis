use std::any::Any;

use crate::hsl::*;
use crate::layers::big_elements;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct BackgroundStraightSplit;

impl Layer for BackgroundStraightSplit {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the two rectangles that will make up the straight split background
        let mut rectangle1 = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "50%")
            .set("height", "100%");

        let mut rectangle2 = Rectangle::new()
            .set("x", 500)
            .set("y", 0)
            .set("width", "50%")
            .set("height", "100%");

        // Possibly apply a rotation
        if random.next_bool() {
            rectangle1 = rectangle1.set("transform", "rotate(90, 500, 500)");
            rectangle2 = rectangle2.set("transform", "rotate(90, 500, 500)");
        }

        // Pick either solid or gradient colors
        if random.roll::<u8>(100) < 80 {
            // Solid colors
            let (color1, color2) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 15 {
                    ColorMode::Tone
                } else if roll < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            // Add the fill to the rectangles
            rectangle1 = rectangle1.set("fill", color1);
            rectangle2 = rectangle2.set("fill", color2);

            vec![rectangle1.into(), rectangle2.into()]
        } else {
            // Gradients
            let ((gradient1, gradient1_name), (gradient2, gradient2_name)) = if base_color.is_some()
            {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, Some(45), color1, color2),
                    gradient_definition(random, Some(45), color3, color4),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 15 {
                    ColorMode::Tone
                } else if roll < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            // Add the fill to the rectangles
            rectangle1 = rectangle1.set("fill", format!("url(#{gradient1_name})"));
            rectangle2 = rectangle2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                rectangle1.into(),
                rectangle2.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::big_element_two_squares::BigElementTwoSquares.type_id(), // The two squares big element doesn't differentiate enough from this background
        ]
    }
}
