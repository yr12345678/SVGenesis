use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Circle, Element};

pub struct SmallElementFourCircles;

impl Layer for SmallElementFourCircles {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(50, 100);

        // Possibly add a rotation
        let valid_rotate_amounts = [0, 45, 90, 135, 180, 225, 270, 315];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(8))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the circles
        let mut circle1 = Circle::new()
            .set("cx", 500 - random_radius)
            .set("cy", 500 + random_radius)
            .set("r", random_radius)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        let mut circle2 = Circle::new()
            .set("cx", 500 + random_radius)
            .set("cy", 500 + random_radius)
            .set("r", random_radius)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        let mut circle3 = Circle::new()
            .set("cx", 500 - random_radius)
            .set("cy", 500 - random_radius)
            .set("r", random_radius)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        let mut circle4 = Circle::new()
            .set("cx", 500 + random_radius)
            .set("cy", 500 - random_radius)
            .set("r", random_radius)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

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

            circle1 = circle1.set("fill", color.clone());
            circle2 = circle2.set("fill", color.clone());
            circle3 = circle3.set("fill", color.clone());
            circle4 = circle4.set("fill", color);

            vec![
                circle1.into(),
                circle2.into(),
                circle3.into(),
                circle4.into(),
            ]
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

            circle1 = circle1.set("fill", format!("url(#{gradient_name})",));
            circle2 = circle2.set("fill", format!("url(#{gradient_name})",));
            circle3 = circle3.set("fill", format!("url(#{gradient_name})",));
            circle4 = circle4.set("fill", format!("url(#{gradient_name})",));

            vec![
                gradient.into(),
                circle1.into(),
                circle2.into(),
                circle3.into(),
                circle4.into(),
            ]
        }
    }
}
