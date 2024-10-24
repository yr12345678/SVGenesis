use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementCrossShadow;

impl Layer for SmallElementCrossShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Get a random width
        let random_size = random.in_range::<u16>(25, 50) * 6; // Must be divisible by 3 and 2

        // Generate the cross
        let data = Data::new()
            .move_to((500 - random_size / 2, 500))
            .line_to((500 + random_size / 2, 500))
            .move_to((500, 500 - random_size / 2))
            .line_to((500, 500 + random_size / 2));

        let mut cross1 = Path::new()
            .set("d", data)
            .set("stroke-width", random_size / 3);

        let mut cross2 = cross1.clone();

        // Randomly set rotate and translate
        let valid_rotate_amounts = [0, 45];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        cross1 = cross1.set(
            "transform",
            format!("rotate({rotate_amount}, 500, 500) translate(-5, -5)"),
        );
        cross2 = cross2.set(
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
            cross1 = cross1.set("stroke", color.as_string());
            cross2 = cross2.set(
                "stroke",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![cross2.into(), cross1.into()]);
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

            cross1 = cross1.set("stroke", format!("url(#{gradient_name})"));
            cross2 = cross2.set("stroke", shadow_color);

            elements.extend(vec![gradient.into(), cross2.into(), cross1.into()]);
        }

        // Return vector of elements
        elements
    }
}
