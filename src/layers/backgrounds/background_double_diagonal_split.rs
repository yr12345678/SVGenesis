use std::any::Any;

use crate::hsl::*;
use crate::layers::{big_elements, overlays};
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct BackgroundDoubleDiagonalSplit;

impl Layer for BackgroundDoubleDiagonalSplit {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the areas
        let area1 = Data::new()
            .move_to((0, 0))
            .horizontal_line_to(1000)
            .vertical_line_to(500)
            .close();

        let area2 = Data::new()
            .move_to((0, 0))
            .vertical_line_to(500)
            .horizontal_line_to(1000)
            .close();

        let area3 = Data::new()
            .move_to((0, 500))
            .horizontal_line_to(1000)
            .vertical_line_to(1000)
            .close();

        let area4 = Data::new()
            .move_to((0, 500))
            .vertical_line_to(1000)
            .horizontal_line_to(1000)
            .close();

        let mut path1 = Path::new().set("d", area1);
        let mut path2 = Path::new().set("d", area2);
        let mut path3 = Path::new().set("d", area3);
        let mut path4 = Path::new().set("d", area4);

        // Possibly apply a rotation
        if random.next_bool() {
            path1 = path1.set("transform", "rotate(90, 500, 500)");
            path2 = path2.set("transform", "rotate(90, 500, 500)");
            path3 = path3.set("transform", "rotate(90, 500, 500)");
            path4 = path4.set("transform", "rotate(90, 500, 500)");
        }

        // Pick either solid or gradient colors
        if random.roll::<u8>(100) < 80 {
            // Solid colors
            let (color1, color2, color3, color4) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
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
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            // Add the fill
            path1 = path1.set("fill", color1);
            path2 = path2.set("fill", color2);
            path3 = path3.set("fill", color3);
            path4 = path4.set("fill", color4);

            vec![path1.into(), path2.into(), path3.into(), path4.into()]
        } else {
            // Gradients
            let (
                (gradient1, gradient1_name),
                (gradient2, gradient2_name),
                (gradient3, gradient3_name),
                (gradient4, gradient4_name),
            ) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);
                let color5 = base_color.unwrap().derive_similar_color(random);
                let color6 = base_color.unwrap().derive_similar_color(random);
                let color7 = base_color.unwrap().derive_similar_color(random);
                let color8 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, Some(45), color1, color2),
                    gradient_definition(random, Some(45), color3, color4),
                    gradient_definition(random, Some(45), color5, color6),
                    gradient_definition(random, Some(45), color7, color8),
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
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            // Add the fill
            path1 = path1.set("fill", format!("url(#{gradient1_name})"));
            path2 = path2.set("fill", format!("url(#{gradient2_name})"));
            path3 = path3.set("fill", format!("url(#{gradient3_name})"));
            path4 = path4.set("fill", format!("url(#{gradient4_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                gradient3.into(),
                gradient4.into(),
                path1.into(),
                path2.into(),
                path3.into(),
                path4.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::big_element_zig_zag::BigElementZigZag.type_id(),
            big_elements::big_element_pill::BigElementPill.type_id(),
            big_elements::big_element_pill_split_circle::BigElementPillSplitCircle.type_id(),
            big_elements::big_element_pill_ball::BigElementPillBall.type_id(),
            overlays::overlay_triangle::OverlayTriangle.type_id(),
        ]
    }
}
