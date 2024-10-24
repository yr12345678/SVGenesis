use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SmallElementStraightSplitSquare;

impl Layer for SmallElementStraightSplitSquare {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the required values for building the rectangle. It will vary in size
        // and we have to adjust its position and corner radius with it.
        let random_dimension = random.in_range::<u16>(75, 125) * 2;

        // Generate the two rectangles that will make up the straight split background
        let mut rectangle1 = Rectangle::new()
            .set("x", 500 - random_dimension / 2)
            .set("y", 500 - random_dimension / 2)
            .set("width", random_dimension / 2)
            .set("height", random_dimension);

        let mut rectangle2 = Rectangle::new()
            .set("x", 500)
            .set("y", 500 - random_dimension / 2)
            .set("width", random_dimension / 2)
            .set("height", random_dimension);

        // Apply a rotation
        let valid_rotate_amounts = [0, 45, 90, 135];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        rectangle1 = rectangle1.set("transform", format!("rotate({rotate_amount}, 500, 500)"));
        rectangle2 = rectangle2.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

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

            // Add the fill to the rectangles
            rectangle1 = rectangle1.set("fill", color1);
            rectangle2 = rectangle2.set("fill", color2);

            vec![rectangle1.into(), rectangle2.into()]
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

            // Add the fill to the rectangles
            rectangle1 = rectangle1.set("fill", format!("url(#{gradient1_name})"));
            rectangle2 = rectangle2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                rectangle1.into(),
                rectangle2.into(),
            ]
        }
    }
}
