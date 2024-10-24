use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct SmallElementTriangle;

impl Layer for SmallElementTriangle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_offset = random.in_range::<u16>(50, 100) * 2;
        let offset_minus = 500 - random_offset;
        let offset_plus = 500 + random_offset;
        let offset_half_minus = 500 - random_offset / 2;
        let offset_half_plus = 500 + random_offset / 2;

        // Possibly rotate the triangle
        let valid_rotate_amounts = [0, 180];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        let mut triangle = Polygon::new()
            .set("points", format!("{offset_minus},{offset_half_plus} {offset_plus},{offset_half_plus} 500,{offset_half_minus}"))
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

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

            triangle = triangle.set("fill", color);

            elements.push(triangle.into());
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Randomize the color mode, but prefer vibrant
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            triangle = triangle.set("fill", format!("url(#{gradient_name})",));

            elements.extend(vec![gradient.into(), triangle.into()]);
        }

        // Return the vector of elements
        elements
    }
}
