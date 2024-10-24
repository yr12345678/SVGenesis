use crate::hsl::*;
use crate::layers::Layer;
use random::Random;
use svg::node::element::path::Data;
use svg::node::element::{Element, Path, Rectangle};

pub struct SmallElementCube;

impl Layer for SmallElementCube {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_size = random.in_range::<u16>(8, 21) * 12; // Must be divisible by 12 and 2

        let mut rectangle = Rectangle::new()
            .set("x", 500 - random_size / 2 - random_size / 12)
            .set("y", 500 - random_size / 2 + random_size / 12)
            .set("width", random_size)
            .set("height", random_size);

        let data1 = Data::new()
            .move_to((
                500 - random_size / 2 - random_size / 12,
                500 - random_size / 2 + random_size / 12,
            ))
            .line_to((
                500 - random_size / 2 + random_size / 12,
                500 - random_size / 2 - random_size / 12,
            ))
            .line_to((
                500 + random_size / 2 + random_size / 12,
                500 - random_size / 2 - random_size / 12,
            ))
            .line_to((
                500 + random_size / 2 - random_size / 12,
                500 - random_size / 2 + random_size / 12,
            ));

        let data2 = Data::new()
            .move_to((
                500 + random_size / 2 - random_size / 12,
                500 - random_size / 2 + random_size / 12,
            ))
            .line_to((
                500 + random_size / 2 + random_size / 12,
                500 - random_size / 2 - random_size / 12,
            ))
            .line_to((
                500 + random_size / 2 + random_size / 12,
                500 + random_size / 2 - random_size / 12,
            ))
            .line_to((
                500 + random_size / 2 - random_size / 12,
                500 + random_size / 2 + random_size / 12,
            ));

        let mut path1 = Path::new().set("d", data1);
        let mut path2 = Path::new().set("d", data2);

        // Set the fill
        let (color1, color2, color3) = if base_color.is_some() {
            // Use the base color and derive something similar
            let color1 = base_color.unwrap().derive_similar_color(random);
            (
                color1.as_string(),
                HSL {
                    lightness: color1.lightness + 10,
                    ..color1
                }
                .as_string(),
                HSL {
                    lightness: color1.lightness - 10,
                    ..color1
                }
                .as_string(),
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

            (
                color1.as_string(),
                HSL {
                    lightness: color1.lightness + 10,
                    ..color1
                }
                .as_string(),
                HSL {
                    lightness: color1.lightness - 10,
                    ..color1
                }
                .as_string(),
            )
        };

        rectangle = rectangle.set("fill", color1);
        path1 = path1.set("fill", color2);
        path2 = path2.set("fill", color3);

        vec![rectangle.into(), path1.into(), path2.into()]
    }
}
