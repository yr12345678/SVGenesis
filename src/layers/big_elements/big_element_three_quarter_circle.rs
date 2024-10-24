use std::any::Any;

use crate::hsl::*;
use crate::layers::overlays;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{path::Data, Element, Path};

pub struct BigElementThreeQuarterCircle;

impl Layer for BigElementThreeQuarterCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Randomly pick a direction
        let data = match random.roll::<u8>(4) {
            0 => {
                // Bottom-left quarter cut
                Data::new()
                    .move_to((0, 500))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 500, 1000))
                    .line_to((500, 500))
                    .close()
            }
            1 => {
                // Top-left quarter cut
                Data::new()
                    .move_to((500, 0))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 0, 500))
                    .line_to((500, 500))
                    .close()
            }
            2 => {
                // Top-right quarter cut
                Data::new()
                    .move_to((1000, 500))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 500, 0))
                    .line_to((500, 500))
                    .close()
            }
            3 => {
                // Bottom-right quarter cut
                Data::new()
                    .move_to((500, 1000))
                    .elliptical_arc_to((500, 500, 0, 1, 1, 1000, 500))
                    .line_to((500, 500))
                    .close()
            }
            _ => panic!("Invalid circle variant"),
        };

        let mut path = Path::new().set("d", data);

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

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![overlays::overlay_half_circle::OverlayHalfCircle.type_id()]
    }
}
