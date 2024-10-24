use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct SmallElementPillShadow;

impl Layer for SmallElementPillShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the pill
        let random_width = random.in_range::<u16>(35, 75) * 4;

        let mut pill = Rectangle::new()
            .set("width", random_width)
            .set("height", random_width / 2)
            .set("x", 500 - random_width / 2)
            .set("y", 500 - random_width / 4)
            .set("rx", random_width / 4)
            .set("transform", "translate(-5, -5)");

        let mut pill_shadow = pill.clone().set("transform", "translate(5, 5)");

        // Possibly add a 90 degree rotation
        if random.next_bool() {
            pill = pill.set("transform", "rotate(90, 500, 500) translate(-5, 5)");
            pill_shadow = pill_shadow.set("transform", "rotate(90, 500, 500) translate(5, -5)");
        }

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
            pill = pill.set("fill", color.as_string());
            pill_shadow = pill_shadow.set(
                "fill",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![pill_shadow.into(), pill.into()]);
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

            pill = pill.set("fill", format!("url(#{gradient_name})"));
            pill_shadow = pill_shadow.set("fill", shadow_color);

            elements.extend(vec![gradient.into(), pill_shadow.into(), pill.into()]);
        }

        // Return vector of elements
        elements
    }
}
