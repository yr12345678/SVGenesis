use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementSplitCircle;

impl Layer for SmallElementSplitCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Get a random radius
        let random_radius = random.in_range::<u16>(50, 100) * 2; // Always an even number

        // Generate the half circles
        let data1 = Data::new()
            .move_to((500 - random_radius, 500))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 + random_radius, 500));

        let data2 = Data::new()
            .move_to((500 - random_radius, 500))
            .elliptical_arc_to((50, 50, 0, 0, 0, 500 + random_radius, 500));

        // Possible add a rotation
        let valid_rotate_amounts = [0, 45, 90, 135];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the paths
        let mut path1 = Path::new()
            .set("d", data1)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        let mut path2 = Path::new()
            .set("d", data2)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Pick random solid colors
        if random.roll::<u8>(100) < 85 {
            // Solid colors
            let (color1, color2) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Pick random colors
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

            // Add the fill to the paths
            path1 = path1.set("fill", color1);
            path2 = path2.set("fill", color2);

            vec![path1.into(), path2.into()]
        } else {
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
                // Generate random gradients
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

            path1 = path1.set("fill", format!("url(#{gradient1_name})"));
            path2 = path2.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                path1.into(),
                path2.into(),
            ]
        }
    }
}
