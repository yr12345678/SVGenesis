use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SmallElementPill;

impl Layer for SmallElementPill {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the pill
        let random_width = random.in_range::<u16>(35, 75) * 4;

        let mut rectangle = Rectangle::new()
            .set("width", random_width)
            .set("height", random_width / 2)
            .set("x", 500 - random_width / 2)
            .set("y", 500 - random_width / 4)
            .set("rx", random_width / 4);

        // Possibly add a 90 degree rotation
        if random.next_bool() {
            rectangle = rectangle.set("transform", "rotate(90, 500, 500)");
        }

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

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

            elements.push(rectangle.into());
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

            elements.extend(vec![gradient.into(), rectangle.into()]);
        }

        // Return vector of elements
        elements
    }
}
