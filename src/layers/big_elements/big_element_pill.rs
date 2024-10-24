use std::any::Any;

use crate::layers::Layer;
use crate::utils::*;
use crate::{hsl::*, layers::small_elements};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct BigElementPill;

impl Layer for BigElementPill {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Build the rectangle
        let mut rectangle = Rectangle::new()
            .set("width", 500)
            .set("height", 1000)
            .set("x", 0)
            .set("y", 0)
            .set("rx", 250)
            .set("ry", 250);

        // Add a rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        rectangle = rectangle.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            rectangle = rectangle.set("fill", color);

            vec![rectangle.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            rectangle = rectangle.set("fill", format!("url(#{gradient_name})",));

            vec![gradient.into(), rectangle.into()]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![small_elements::small_element_cube::SmallElementCube.type_id()]
    }
}
