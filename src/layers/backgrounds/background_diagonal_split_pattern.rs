use std::any::Any;

use crate::layers::Layer;
use crate::utils::*;
use crate::{hsl::*, layers::small_elements};
use random::Random;
use svg::node::element::{Definitions, Element, Pattern, Polygon, Rectangle};

pub struct BackgroundDiagonalSplitPattern;

impl Layer for BackgroundDiagonalSplitPattern {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the rectangle that will be our background
        let mut pattern_rectangle = Rectangle::new().set("width", "100%").set("height", "100%");

        // Generate the polygon that will form the diagonal split
        let mut polygon = Polygon::new().set("points", "0,0 250,250 0,250");

        // Generate the pattern
        let pattern_name = format!("pat{}", random.in_range::<u16>(0, 65535));
        let mut pattern = Pattern::new()
            .set("id", pattern_name.clone())
            .set("patternUnits", "userSpaceOnUse")
            .set("width", 250)
            .set("height", 250);

        // Set rotation
        let valid_rotate_amounts = [0, 90];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        pattern = pattern.set("patternTransform", format!("rotate({rotate_amount})"));

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

            pattern_rectangle = pattern_rectangle.set("fill", color1);
            polygon = polygon.set("fill", color2);
            pattern = pattern.add(pattern_rectangle).add(polygon);

            let defs = Definitions::new().add(pattern);

            // Create a rectangle with that pattern, which serves as the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", format!("url(#{pattern_name})"));

            vec![defs.into(), background.into()]
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

            pattern_rectangle = pattern_rectangle.set("fill", format!("url(#{gradient1_name})"));
            polygon = polygon.set("fill", format!("url(#{gradient2_name})"));
            pattern = pattern.add(pattern_rectangle).add(polygon);

            let defs = Definitions::new()
                .add(gradient1)
                .add(gradient2)
                .add(pattern);

            // Create a rectangle with that pattern, which serves as the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", format!("url(#{pattern_name})"));

            vec![defs.into(), background.into()]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            small_elements::small_element_diagonal_split_square::SmallElementDiagonalSplitSquare
                .type_id(),
            small_elements::small_element_triangle::SmallElementTriangle.type_id(),
            small_elements::small_element_square::SmallElementSquare.type_id(),
            small_elements::small_element_square_shadow::SmallElementSquareShadow.type_id(),
        ]
    }
}