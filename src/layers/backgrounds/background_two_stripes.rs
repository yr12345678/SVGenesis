use std::any::Any;

use crate::layers::{overlays, Layer};
use crate::{hsl::*, layers::big_elements};
use random::Random;
use svg::node::element::{Definitions, Element, Pattern, Rectangle};

pub struct BackgroundTwoStripes;

impl Layer for BackgroundTwoStripes {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the colors for the stripes, we ignore one color as that's a bit easier
        // with the color generation methods.
        let (color1, color2, _) = if base_color.is_some() {
            // We use the base color for everything
            match random.roll::<u8>(3) {
                0 => (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                ),
                1 => base_color.unwrap().analogous_colors_as_strings(),
                2 => base_color.unwrap().monochromatic_colors_as_strings(),
                _ => panic!("Invalid color variant"),
            }
        } else {
            // Pick a random color
            let roll = random.roll::<u8>(100);
            let color_mode = if roll < 20 {
                ColorMode::Tone
            } else if roll < 50 {
                ColorMode::Light
            } else {
                ColorMode::Vibrant
            };

            match random.roll::<u8>(3) {
                0 => (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                ),
                1 => HSL::new_random(random, color_mode, 100).analogous_colors_as_strings(),
                2 => HSL::new_random(random, color_mode, 100).monochromatic_colors_as_strings(),
                _ => panic!("Invalid color variant"),
            }
        };

        // Randomly set rotation
        let valid_rotate_amounts = [-45, 0, 45, 90];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the stripes
        let rectangle1 = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("height", 200)
            .set("width", "100%")
            .set("fill", color1);

        let rectangle2 = Rectangle::new()
            .set("x", 0)
            .set("y", 200)
            .set("height", 200)
            .set("width", "100%")
            .set("fill", color2);

        // Add the stripes to a pattern an add that to the definitions
        let translate_amount = match *rotate_amount {
            // This is a dirty fix for aligning the lines neatly with the corners
            45 => "7",
            -45 => "0, 7",
            _ => "0",
        };
        let pattern_name = format!("pat{}", random.in_range::<u16>(0, 65535));
        let pattern = Pattern::new()
            .set("id", pattern_name.clone())
            .set(
                "patternTransform",
                format!("rotate({rotate_amount}) translate({translate_amount})"),
            )
            .set("patternUnits", "userSpaceOnUse")
            .set("width", "100%")
            .set("height", 400)
            .add(rectangle1)
            .add(rectangle2);

        let defs = Definitions::new().add(pattern);

        // Create a rectangle with that pattern, which serves as the background
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", format!("url(#{pattern_name})"));

        vec![defs.into(), background.into()]
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            big_elements::big_element_zig_zag::BigElementZigZag.type_id(),
            big_elements::big_element_pill::BigElementPill.type_id(),
            big_elements::big_element_pill_split_circle::BigElementPillSplitCircle.type_id(),
            big_elements::big_element_pill_ball::BigElementPillBall.type_id(),
            overlays::overlay_triangle::OverlayTriangle.type_id(),
        ]
    }
}
