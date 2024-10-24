use std::any::Any;

use crate::hsl::*;
use crate::layers::small_elements;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Circle, Definitions, Element, Pattern, Rectangle};

pub struct BackgroundCirclePattern;

impl Layer for BackgroundCirclePattern {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the rectangle that will be our background color
        let mut pattern_rectangle = Rectangle::new().set("width", "100%").set("height", "100%");

        // Generate the circle
        let mut circle = Circle::new().set("cx", 100).set("cy", 100).set("r", 100);

        // Generate the pattern
        let pattern_name = format!("pat{}", random.in_range::<u16>(0, 65535));
        let mut pattern = Pattern::new()
            .set("id", pattern_name.clone())
            .set("patternUnits", "userSpaceOnUse")
            .set("width", 200)
            .set("height", 200);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let (color1, color2) = if base_color.is_some() {
                // Use the base color
                (
                    base_color.unwrap().derive_similar_color(random).as_string(),
                    base_color.unwrap().derive_similar_color(random).as_string(),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 15 {
                    ColorMode::Tone
                } else if roll < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    HSL::new_random(random, color_mode, 100).as_string(),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            pattern_rectangle = pattern_rectangle.set("fill", color1);
            circle = circle.set("fill", color2);
            pattern = pattern.add(pattern_rectangle).add(circle);

            let defs = Definitions::new().add(pattern);

            // Create a rectangle with that pattern, which serves as the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", format!("url(#{pattern_name})"));

            vec![defs.into(), background.into()]
        } else {
            // Get a gradient definition
            let ((gradient1, gradient1_name), rect_color) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let rect_color = base_color.unwrap().derive_similar_color(random).as_string();

                (
                    gradient_definition(random, Some(45), color1, color2),
                    rect_color,
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 15 {
                    ColorMode::Tone
                } else if roll < 50 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    HSL::new_random(random, color_mode, 100).as_string(),
                )
            };

            pattern_rectangle = pattern_rectangle.set("fill", format!("url(#{gradient1_name})"));
            circle = circle.set("fill", rect_color);
            pattern = pattern.add(pattern_rectangle).add(circle);

            let defs = Definitions::new().add(gradient1).add(pattern);

            // Create a rectangle with that pattern, which serves as the background
            let background = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", format!("url(#{pattern_name})"));

            vec![defs.into(), background.into()]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            small_elements::small_element_circle::SmallElementCircle.type_id(),
            small_elements::small_element_circle_shadow::SmallElementCircleShadow.type_id(),
            small_elements::small_element_double_circle::SmallElementDoubleCircle.type_id(),
            small_elements::small_element_split_circle::SmallElementSplitCircle.type_id(),
        ]
    }
}
