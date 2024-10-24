use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path};

pub struct SmallElementStarShadow;

impl Layer for SmallElementStarShadow {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Pick a radius
        let valid_radii = [90, 120, 150];
        let radius = valid_radii
            .get(random.roll::<usize>(3))
            .expect("Did not find a valid radius. This should never happen.");

        // Generate the star
        let data = Data::new()
            .move_to((500 - radius, 500))
            .line_to((500 - radius / 3, 500 - radius / 3))
            .line_to((500, 500 - radius))
            .line_to((500 + radius / 3, 500 - radius / 3))
            .line_to((500 + radius, 500))
            .line_to((500 + radius / 3, 500 + radius / 3))
            .line_to((500, 500 + radius))
            .line_to((500 - radius / 3, 500 + radius / 3));

        let mut star = Path::new()
            .set("d", data)
            .set("transform", "translate(-5, -5)");

        let mut star_shadow = star.clone().set("transform", "translate(5, 5)");

        // Possibly rotate
        if random.next_bool() {
            star = star.set("transform", "rotate(45, 500, 500) translate(-5, -5)");
            star_shadow = star_shadow.set("transform", "rotate(45, 500, 500) translate(5, 5)");
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

            star = star.set("fill", color.as_string());
            star_shadow = star_shadow.set(
                "fill",
                HSL {
                    lightness: color.lightness - 10,
                    ..color
                }
                .as_string(),
            );

            elements.extend(vec![star_shadow.into(), star.into()]);
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

            star = star.set("fill", format!("url(#{gradient_name})",));
            star_shadow = star_shadow.set("fill", shadow_color);

            elements.extend(vec![gradient.into(), star_shadow.into(), star.into()]);
        }

        // Return the vector of elements
        elements
    }
}
