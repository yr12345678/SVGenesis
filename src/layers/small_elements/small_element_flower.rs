use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementFlower;

impl Layer for SmallElementFlower {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Get a random size
        let random_size = random.in_range::<u16>(25, 50) * 2; // Always an even number

        // Generate the half circles
        let data = Data::new()
            .move_to((500 - random_size, 500 - random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 + random_size, 500 - random_size))
            .move_to((500 + random_size, 500 - random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 + random_size, 500 + random_size))
            .move_to((500 + random_size, 500 + random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 - random_size, 500 + random_size))
            .move_to((500 - random_size, 500 + random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 - random_size, 500 - random_size))
            .move_to((500 - random_size, 500 - random_size))
            .horizontal_line_to(500 + random_size)
            .vertical_line_to(500 + random_size)
            .horizontal_line_to(500 - random_size)
            .close();

        // Possibly add a rotation
        let valid_rotate_amounts = [0, 45];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the path
        let mut path = Path::new()
            .set("d", data)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Pick random solid colors
        if random.roll::<u8>(100) < 85 {
            // Solid colors
            let color = if base_color.is_some() {
                // Use the base color
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

            // Add the fill to the paths
            path = path.set("fill", color);

            elements.push(path.into());
        } else {
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

            path = path.set("fill", format!("url(#{gradient_name})"));

            elements.extend(vec![gradient.into(), path.into()]);
        }

        // Return the vector of elements
        elements
    }
}
