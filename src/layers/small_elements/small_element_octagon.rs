use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct SmallElementOctagon;

impl Layer for SmallElementOctagon {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_size = random.in_range::<u16>(30, 60) * 4;
        let offset_half_minus = 500 - random_size / 2;
        let offset_half_plus = 500 + random_size / 2;
        let offset_quarter_minus = 500 - random_size / 4;
        let offset_quarter_plus = 500 + random_size / 4;

        let mut octagon = Polygon::new().set(
            "points",
            format!(
                "{offset_half_minus},{offset_quarter_plus} {offset_half_minus},{offset_quarter_minus} {offset_quarter_minus},{offset_half_minus} {offset_quarter_plus},{offset_half_minus} {offset_half_plus},{offset_quarter_minus} {offset_half_plus},{offset_quarter_plus} {offset_quarter_plus},{offset_half_plus} {offset_quarter_minus},{offset_half_plus}"
            ),
        );

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

            octagon = octagon.set("fill", color);

            elements.push(octagon.into());
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

            octagon = octagon.set("fill", format!("url(#{gradient_name})",));

            elements.extend(vec![gradient.into(), octagon.into()]);
        }

        // Return vector of elements
        elements
    }
}
