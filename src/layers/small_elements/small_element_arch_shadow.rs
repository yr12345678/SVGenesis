use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementArchShadow;

impl Layer for SmallElementArchShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_width = random.in_range::<u16>(50, 100);
        let radius = random_width / 2; // Will get rounded, but no big issue

        let data = Data::new()
            .move_to((500 - random_width, 500 + random_width + radius))
            .line_to((500 - random_width, 500 - radius))
            .line_to((500 + random_width, 500 - radius))
            .line_to((500 + random_width, 500 + random_width + radius))
            .move_to((500 - random_width, 500 - radius))
            .elliptical_arc_to((radius, radius, 0, 0, 1, 500 + random_width, 500 - radius));

        let mut arch = Path::new()
            .set("d", data)
            .set("transform", "translate(-5, -5)");

        let mut arch_shadow = arch.clone().set("transform", "translate(5, 5)");

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

            arch = arch.set("fill", color.as_string());
            arch_shadow = arch_shadow.set(
                "fill",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![arch_shadow.into(), arch.into()]);
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

            arch = arch.set("fill", format!("url(#{gradient_name})",));
            arch_shadow = arch_shadow.set("fill", shadow_color);

            elements.extend(vec![gradient.into(), arch_shadow.into(), arch.into()]);
        }

        // Return vector of elements
        elements
    }
}
