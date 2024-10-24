use std::any::Any;

use crate::layers::Layer;
use crate::utils::*;
use crate::{hsl::*, layers::big_elements};
use random::Random;
use svg::node::element::{Element, Rectangle};

pub struct FrameStraight;

impl Layer for FrameStraight {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Randomly set stroke width
        let valid_stroke_widths = [100]; // Should be divisable by 2
        let stroke_width = valid_stroke_widths
            .get(random.roll::<usize>(1))
            .expect("Did not find a valid stroke width. This should never happen.");

        // Generate the rectangle that will be our background
        let mut rectangle = Rectangle::new()
            .set("stroke-width", *stroke_width)
            .set("fill", "none")
            .set("x", *stroke_width / 2)
            .set("y", *stroke_width / 2)
            .set("width", 1000 - *stroke_width)
            .set("height", 1000 - *stroke_width);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a darker solid color
            let color = if base_color.is_some() {
                let unwrapped = base_color.unwrap();

                HSL {
                    lightness: unwrapped.lightness - 30,
                    ..unwrapped
                }
                .as_string()
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            rectangle = rectangle.set("stroke", color);

            vec![rectangle.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let unwrapped = base_color.unwrap();

                let color1 = HSL {
                    lightness: unwrapped.lightness - 15,
                    ..unwrapped
                };
                let color2 = HSL {
                    lightness: unwrapped.lightness - 30,
                    ..unwrapped
                };

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            rectangle = rectangle.set("stroke", format!("url(#{gradient_name})",));

            vec![gradient.into(), rectangle.into()]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        // Prevent the frame from having any big elements on top
        vec![
            big_elements::big_element_square::BigElementSquare.type_id(),
            big_elements::big_element_full_circle::BigElementFullCircle.type_id(),
            big_elements::big_element_half_circle::BigElementHalfCircle.type_id(),
            big_elements::big_element_quarter_circle::BigElementQuarterCircle.type_id(),
            big_elements::big_element_three_quarter_circle::BigElementThreeQuarterCircle.type_id(),
            big_elements::big_element_triangle::BigElementTriangle.type_id(),
            big_elements::big_element_two_squares::BigElementTwoSquares.type_id(),
            big_elements::big_element_zig_zag::BigElementZigZag.type_id(),
            big_elements::big_element_pill::BigElementPill.type_id(),
            big_elements::big_element_pill_split_circle::BigElementPillSplitCircle.type_id(),
            big_elements::big_element_two_rectangles::BigElementTwoRectangles.type_id(),
        ]
    }
}
