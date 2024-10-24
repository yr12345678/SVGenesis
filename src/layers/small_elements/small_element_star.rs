use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementStar;

impl Layer for SmallElementStar {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Pick a radius
        let valid_radii = [90, 120, 150];
        let radius = valid_radii
            .get(random.roll::<usize>(3))
            .expect("Did not find a valid radius. This should never happen.");

        // Generate the star
        let data = Data::new()
            .move_to((500 - radius, 500))
            .line_to((500 - radius / 3, 500 - radius / 3))
            .line_to((500, 500 - radius))
            .line_to((500 + radius / 3, 500 - radius / 3))
            .line_to((500 + radius, 500))
            .line_to((500 + radius / 3, 500 + radius / 3))
            .line_to((500, 500 + radius))
            .line_to((500 - radius / 3, 500 + radius / 3));

        let mut path = Path::new().set("d", data);

        // Possibly rotate
        if random.next_bool() {
            path = path.set("transform", "rotate(45, 500, 500)");
        }

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color
                let color_mode = if random.roll::<u8>(100) < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            path = path.set("fill", color);

            elements.push(path.into());
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

            path = path.set("fill", format!("url(#{gradient_name})",));

            elements.extend(vec![gradient.into(), path.into()]);
        }

        // Return the vector of elements
        elements
    }
}
