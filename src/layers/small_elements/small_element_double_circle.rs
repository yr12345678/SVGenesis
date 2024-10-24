use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Circle, Element};

pub struct SmallElementDoubleCircle;

impl Layer for SmallElementDoubleCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(25, 45) * 4;

        let mut circle1 = Circle::new()
            .set("cx", 500 - random_radius / 4)
            .set("cy", 500)
            .set("r", random_radius);

        let mut circle2 = Circle::new()
            .set("cx", 500 + random_radius / 4)
            .set("cy", 500)
            .set("r", random_radius);

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
            let (color1, color2) = if base_color.is_some() {
                // Use the base color and derive something similar
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            circle1 = circle1.set("fill", color1);
            circle2 = circle2.set("fill", color2);

            vec![circle1.into(), circle2.into()]
        } else {
            // Get a gradient definition
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
                // Randomize the color mode, but prefer vibrant
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            circle1 = circle1.set("fill", format!("url(#{gradient1_name})"));
            circle2 = circle2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                circle1.into(),
                circle2.into(),
            ]
        }
    }
}
