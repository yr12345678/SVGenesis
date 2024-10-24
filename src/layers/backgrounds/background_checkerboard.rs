use std::any::Any;

use crate::layers::Layer;
use crate::utils::*;
use crate::{hsl::*, layers::overlays};
use random::Random;
use svg::node::element::{path::Data, Element, Path, Rectangle};

pub struct BackgroundCheckerboard;

impl Layer for BackgroundCheckerboard {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the rectangle that will be our background
        let mut rectangle = Rectangle::new().set("width", "100%").set("height", "100%");

        // Generate the path that will form the checkerboard
        let data = Data::new()
            .move_to((0, 0))
            .vertical_line_to(1000)
            .horizontal_line_to(250)
            .vertical_line_to(0)
            .close()
            .move_to((500, 0))
            .vertical_line_to(1000)
            .horizontal_line_to(750)
            .vertical_line_to(0)
            .close()
            .move_to((0, 0))
            .horizontal_line_to(1000)
            .vertical_line_to(250)
            .horizontal_line_to(0)
            .close()
            .move_to((0, 500))
            .horizontal_line_to(1000)
            .vertical_line_to(750)
            .horizontal_line_to(0)
            .close();

        let mut path = Path::new().set("d", data).set("fill-rule", "evenodd");

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let (color1, color2) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 15 {
                    ColorMode::Tone
                } else if roll < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            rectangle = rectangle.set("fill", color1);
            path = path.set("fill", color2);

            vec![rectangle.into(), path.into()]
        } else {
            // Get a gradient definition
            let ((gradient1, gradient1_name), (gradient2, gradient2_name)) = if base_color.is_some()
            {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, Some(45), color1, color2),
                    gradient_definition(random, Some(45), color3, color4),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 15 {
                    ColorMode::Tone
                } else if roll < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            rectangle = rectangle.set("fill", format!("url(#{gradient1_name})"));
            path = path.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                rectangle.into(),
                path.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![overlays::overlay_triangle::OverlayTriangle.type_id()]
    }
}
