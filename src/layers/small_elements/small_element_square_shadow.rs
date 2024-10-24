use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SmallElementSquareShadow;

impl Layer for SmallElementSquareShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the required values for building the rectangle. It will vary in size
        // and we have to adjust its position and corner radius with it.
        let random_dimension = random.in_range::<u16>(75, 125) * 2;
        let rx = random_dimension / 5; // This will just get rounded, which is fine
        let position = 500 - (random_dimension / 2);

        // Build the rectangle
        let mut rectangle = Rectangle::new()
            .set("width", random_dimension)
            .set("height", random_dimension)
            .set("x", position)
            .set("y", position);

        // The shadow
        let mut rectangle_shadow = rectangle.clone();

        // Add rotation and translate
        let valid_rotate_amounts = [0, 45];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(2))
            .expect("Did not find a valid rotation amount. This should never happen.");

        rectangle = rectangle.set(
            "transform",
            format!("rotate({rotate_amount}, 500, 500) translate(-5, -5)"),
        );

        rectangle_shadow = rectangle_shadow.set(
            "transform",
            format!("rotate({rotate_amount}, 500, 500) translate(5, 5)"),
        );

        // Possibly add rounded corners
        if random.next_bool() {
            rectangle = rectangle.set("rx", rx);
            rectangle_shadow = rectangle_shadow.set("rx", rx);
        }

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random)
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100)
            };

            rectangle = rectangle.set("fill", color.as_string());
            rectangle_shadow = rectangle_shadow.set(
                "fill",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![rectangle_shadow.into(), rectangle.into()]);
        } else {
            // Get a gradient definition
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
                // Pick a random color
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

            rectangle = rectangle.set("fill", format!("url(#{gradient_name})",));
            rectangle_shadow = rectangle_shadow.set("fill", shadow_color);

            elements.extend(vec![
                gradient.into(),
                rectangle_shadow.into(),
                rectangle.into(),
            ]);
        }

        // Return the vector of elements
        elements
    }
}
