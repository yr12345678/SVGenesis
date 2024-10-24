use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct SmallElementOctagonShadow;

impl Layer for SmallElementOctagonShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_size = random.in_range::<u16>(30, 60) * 4;
        let offset_half_minus = 500 - random_size / 2;
        let offset_half_plus = 500 + random_size / 2;
        let offset_quarter_minus = 500 - random_size / 4;
        let offset_quarter_plus = 500 + random_size / 4;

        let mut octagon = Polygon::new()
            .set(
                "points",
                format!(
                    "{offset_half_minus},{offset_quarter_plus} {offset_half_minus},{offset_quarter_minus} {offset_quarter_minus},{offset_half_minus} {offset_quarter_plus},{offset_half_minus} {offset_half_plus},{offset_quarter_minus} {offset_half_plus},{offset_quarter_plus} {offset_quarter_plus},{offset_half_plus} {offset_quarter_minus},{offset_half_plus}"
                ),
            )
            .set("transform", "translate(-5, -5)");

        let mut octagon_shadow = octagon.clone().set("transform", "translate(5, 5)");

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
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

            octagon = octagon.set("fill", color.as_string());
            octagon_shadow = octagon_shadow.set(
                "fill",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![octagon_shadow.into(), octagon.into()]);
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
                // Randomize the color mode, but prefer vibrant
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

            octagon = octagon.set("fill", format!("url(#{gradient_name})",));
            octagon_shadow = octagon_shadow.set("fill", shadow_color);

            elements.extend(vec![gradient.into(), octagon_shadow.into(), octagon.into()]);
        }

        // Return vector of elements
        elements
    }
}
