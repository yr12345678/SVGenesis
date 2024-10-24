use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{path::Data, Element, Path};

pub struct BigElementQuarterCircle;

impl Layer for BigElementQuarterCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Pick a position
        let data = match random.roll::<u8>(4) {
            0 => {
                // Top-left
                Data::new()
                    .move_to((0, 771))
                    .elliptical_arc_to((1000, 1000, 0, 0, 0, 771, 0))
                    .line_to((0, 0))
                    .close()
            }
            1 => {
                // Top-right
                Data::new()
                    .move_to((229, 0))
                    .elliptical_arc_to((1000, 1000, 0, 0, 0, 1000, 771))
                    .line_to((1000, 0))
                    .close()
            }
            2 => {
                // Bottom-right
                Data::new()
                    .move_to((1000, 229))
                    .elliptical_arc_to((1000, 1000, 0, 0, 0, 229, 1000))
                    .line_to((1000, 1000))
                    .close()
            }
            3 => {
                // Bottom-left
                Data::new()
                    .move_to((771, 1000))
                    .elliptical_arc_to((1000, 1000, 0, 0, 0, 0, 229))
                    .line_to((0, 1000))
                    .close()
            }
            _ => panic!("Unknown direction"),
        };

        let mut path = Path::new().set("d", data);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let color = if base_color.is_some() {
                // We have a base color, so we derive something similar
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

            path = path.set("fill", color);

            vec![path.into()]
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

            vec![gradient.into(), path.into()]
        }
    }
}
