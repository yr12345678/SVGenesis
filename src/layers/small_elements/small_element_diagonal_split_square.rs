use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct SmallElementDiagonalSplitSquare;

impl Layer for SmallElementDiagonalSplitSquare {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the two triangles that will make up the diagonally split square
        let random_offset = random.in_range::<u16>(75, 125);
        let offset_minus = 500 - random_offset;
        let offset_plus = 500 + random_offset;

        let mut triangle1 = Polygon::new().set("points", format!("{offset_minus},{offset_minus} {offset_plus},{offset_minus} {offset_minus},{offset_plus}"));
        let mut triangle2 = Polygon::new().set("points", format!("{offset_minus},{offset_plus} {offset_plus},{offset_plus} {offset_plus},{offset_minus}"));

        // Add a rotation
        let valid_rotate_amounts = [0, 45, 90, 135];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        triangle1 = triangle1.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
        triangle2 = triangle2.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

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
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            // Add the fill to the triangles
            triangle1 = triangle1.set("fill", color1);
            triangle2 = triangle2.set("fill", color2);

            vec![triangle1.into(), triangle2.into()]
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
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            // Add the fill to the triangles
            triangle1 = triangle1.set("fill", format!("url(#{gradient1_name})"));
            triangle2 = triangle2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                triangle1.into(),
                triangle2.into(),
            ]
        }
    }
}
