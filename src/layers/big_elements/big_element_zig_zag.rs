use std::any::Any;

use crate::layers::{overlays, Layer};
use crate::utils::*;
use crate::{hsl::*, layers::frames};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct BigElementZigZag;

impl Layer for BigElementZigZag {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Randomly pick a rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the zig zag path
        let data = Data::new()
            .move_to((0, 0))
            .line_to((0, 150))
            .line_to((100, 250))
            .line_to((200, 150))
            .line_to((300, 250))
            .line_to((400, 150))
            .line_to((500, 250))
            .line_to((600, 150))
            .line_to((700, 250))
            .line_to((800, 150))
            .line_to((900, 250))
            .line_to((1000, 150))
            .line_to((1000, 0))
            .close();

        let mut path = Path::new()
            .set("d", data)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

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
            // Randomly pick a gradient rotation
            let valid_gradient_rotate_amounts: [u16; 3] = [0, 45, 90];
            let gradient_rotate_amount = valid_gradient_rotate_amounts
                .get(random.roll::<usize>(3))
                .expect("Did not find a valid rotation amount. This should never happen.");

            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(*gradient_rotate_amount), color1, color2)
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(*gradient_rotate_amount), color_mode, 100)
            };

            path = path.set("fill", format!("url(#{gradient_name})",));

            vec![gradient.into(), path.into()]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            frames::frame_straight::FrameStraight.type_id(),
            overlays::overlay_triangle::OverlayTriangle.type_id(),
        ]
    }
}
