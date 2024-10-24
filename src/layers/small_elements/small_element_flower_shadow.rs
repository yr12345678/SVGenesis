use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementFlowerShadow;

impl Layer for SmallElementFlowerShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Get a random size
        let random_size = random.in_range::<u16>(25, 50) * 2; // Always an even number

        // Generate the half circles
        let data = Data::new()
            .move_to((500 - random_size, 500 - random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 + random_size, 500 - random_size))
            .move_to((500 + random_size, 500 - random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 + random_size, 500 + random_size))
            .move_to((500 + random_size, 500 + random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 - random_size, 500 + random_size))
            .move_to((500 - random_size, 500 + random_size))
            .elliptical_arc_to((50, 50, 0, 0, 1, 500 - random_size, 500 - random_size))
            .move_to((500 - random_size, 500 - random_size))
            .horizontal_line_to(500 + random_size)
            .vertical_line_to(500 + random_size)
            .horizontal_line_to(500 - random_size)
            .close();

        // Possibly add a rotation
        let valid_rotate_amounts = [0, 45];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the path
        let mut flower = Path::new().set("d", data).set(
            "transform",
            format!("rotate({rotate_amount}, 500, 500) translate(-5, -5)"),
        );

        let mut flower_shadow = flower.clone().set(
            "transform",
            format!("rotate({rotate_amount}, 500, 500) translate(5, 5)"),
        );

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Pick random solid colors
        if random.roll::<u8>(100) < 85 {
            // Solid colors
            let color = if base_color.is_some() {
                // Use the base color
                base_color.unwrap().derive_similar_color(random)
            } else {
                // Pick random colors
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100)
            };

            // Add the fill to the paths
            flower = flower.set("fill", color.as_string());
            flower_shadow = flower_shadow.set(
                "fill",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![flower_shadow.into(), flower.into()]);
        } else {
            let ((gradient, gradient_name), shadow_color) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = HSL {
                    lightness: color2.lightness - 10,
                    ..color2
                }
                .as_string();

                (
                    gradient_definition(random, Some(45), color1, color2),
                    color3,
                )
            } else {
                // Generate random gradients
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                let color1 = HSL::new_random(random, color_mode, 100);
                let color2 = HSL::new_random(random, color_mode, 100);
                let color3 = HSL {
                    lightness: color2.lightness - 10,
                    ..color2
                }
                .as_string();

                (
                    gradient_definition(random, Some(45), color1, color2),
                    color3,
                )
            };

            flower = flower.set("fill", format!("url(#{gradient_name})"));
            flower_shadow = flower_shadow.set("fill", shadow_color);

            elements.extend(vec![gradient.into(), flower_shadow.into(), flower.into()]);
        }

        // Return the vector of elements
        elements
    }
}
