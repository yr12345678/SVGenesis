use std::any::Any;

use crate::hsl::*;
use crate::layers::{big_elements, small_elements};
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct BackgroundPuzzle;

impl Layer for BackgroundPuzzle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the two pieces that will form the background
        let mut piece1 = Polygon::new().set("points", "0,0 666,0 666,500 334,500 334,1000 0,1000");

        let mut piece2 =
            Polygon::new().set("points", "1000,0 666,0 666,500 334,500 334,1000 1000,1000");

        // Set a rotation
        let valid_rotate_amounts = [0, 90];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        piece1 = piece1.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
        piece2 = piece2.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Pick either solid or gradient colors
        if random.roll::<u8>(100) < 80 {
            // Solid colors
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

            // Add the fill
            piece1 = piece1.set("fill", color1);
            piece2 = piece2.set("fill", color2);

            vec![piece1.into(), piece2.into()]
        } else {
            // Gradients
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

            // Add the fill
            piece1 = piece1.set("fill", format!("url(#{gradient1_name})"));
            piece2 = piece2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                piece1.into(),
                piece2.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::big_element_quarter_circle::BigElementQuarterCircle.type_id(),
            big_elements::big_element_two_squares::BigElementTwoSquares.type_id(),
            big_elements::big_element_triangle::BigElementTriangle.type_id(),
            big_elements::big_element_pill::BigElementPill.type_id(),
            big_elements::big_element_pill_split_circle::BigElementPillSplitCircle.type_id(),
            big_elements::big_element_pill_ball::BigElementPillBall.type_id(),
            big_elements::big_element_three_quarter_circle::BigElementThreeQuarterCircle.type_id(),
            big_elements::big_element_half_circle::BigElementHalfCircle.type_id(),
            big_elements::big_element_two_rectangles::BigElementTwoRectangles.type_id(),
            small_elements::small_element_cross::SmallElementCross.type_id(),
            small_elements::small_element_cross_shadow::SmallElementCrossShadow.type_id(),
        ]
    }
}
