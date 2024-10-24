use std::any::Any;

use crate::hsl::*;
use crate::layers::{big_elements, overlays};
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct BackgroundFourWaySplit;

impl Layer for BackgroundFourWaySplit {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the areas
        let mut area1 = Polygon::new().set("points", "0,0 1000,0 500,500");
        let mut area2 = Polygon::new().set("points", "1000,0 1000,1000 500,500");
        let mut area3 = Polygon::new().set("points", "1000,1000 0,1000 500,500");
        let mut area4 = Polygon::new().set("points", "0,1000 0,0 500,500");

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

            // Add the fill to the areas
            area1 = area1.set("fill", color1);
            area2 = area2.set("fill", color2);
            area3 = area3.set("fill", color3);
            area4 = area4.set("fill", color4);

            vec![area1.into(), area2.into(), area3.into(), area4.into()]
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

            // Add the fill to the areas
            area1 = area1.set("fill", format!("url(#{gradient1_name})"));
            area2 = area2.set("fill", format!("url(#{gradient2_name})"));
            area3 = area3.set("fill", format!("url(#{gradient3_name})"));
            area4 = area4.set("fill", format!("url(#{gradient4_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                gradient3.into(),
                gradient4.into(),
                area1.into(),
                area2.into(),
                area3.into(),
                area4.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::big_element_triangle::BigElementTriangle.type_id(),
            big_elements::big_element_two_squares::BigElementTwoSquares.type_id(),
            big_elements::big_element_zig_zag::BigElementZigZag.type_id(),
            overlays::overlay_triangle::OverlayTriangle.type_id(),
        ]
    }
}
