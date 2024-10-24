use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementCross;

impl Layer for SmallElementCross {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Get a random width
        let random_size = random.in_range::<u16>(25, 50) * 6; // Must be divisible by 3 and 2

        // Generate the cross
        let data = Data::new()
            .move_to((500 - random_size / 2, 500))
            .line_to((500 + random_size / 2, 500))
            .move_to((500, 500 - random_size / 2))
            .line_to((500, 500 + random_size / 2));

        let mut path = Path::new()
            .set("d", data)
            .set("stroke-width", random_size / 3);

        // Possibly add a rotation
        if random.next_bool() {
            path = path.set("transform", "rotate(45, 500, 500)");
        };

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Pick random solid colors
        if random.roll::<u8>(100) < 85 {
            // Solid colors
            let color = if base_color.is_some() {
                // Use the base color
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick random colors
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            // Add the fill to the paths
            path = path.set("stroke", color.clone());

            elements.push(path.into());
        } else {
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Generate random gradients
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            path = path.set("stroke", format!("url(#{gradient_name})"));

            elements.extend(vec![gradient.into(), path.into()]);
        }

        // Return vector of elements
        elements
    }
}
