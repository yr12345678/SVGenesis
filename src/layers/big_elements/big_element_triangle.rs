use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct BigElementTriangle;

impl Layer for BigElementTriangle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate a triangle with a random positioning and appropriate gradient rotation
        let mut triangle = match random.roll::<u8>(8) {
            0 => Polygon::new().set("points", "0,0 500,500 0,1000"), // Base to left side
            1 => Polygon::new().set("points", "0,0 500,500 1000,0"), // Base to top
            2 => Polygon::new().set("points", "1000,0 500,500 1000,1000"), // Base to right side
            3 => Polygon::new().set("points", "0,1000 500,500 1000,1000"), // Base to bottom
            4 => Polygon::new().set("points", "500,0 0,500 500,1000"), // Point to left side
            5 => Polygon::new().set("points", "0,500 500,0 1000,500"), // Point to top
            6 => Polygon::new().set("points", "500,0 1000,500 500,1000"), // Point to right
            7 => Polygon::new().set("points", "0,500 500,1000 1000,500"), // Point to bottom
            _ => panic!("No matching triangle variant"),
        };

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
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

            // Add the fill to the triangle
            triangle = triangle.set("fill", color);

            vec![triangle.into()]
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

            // Add the fill to the triangle
            triangle = triangle.set("fill", format!("url(#{gradient_name})"));

            vec![gradient.into(), triangle.into()]
        }
    }
}
