use std::any::Any;

use crate::layers::{overlays, Layer};
use crate::utils::*;
use crate::{hsl::*, layers::small_elements};
use random::Random;
use svg::node::element::{path::Data, Element, Path, Rectangle};

pub struct BigElementPillSplitCircle;

impl Layer for BigElementPillSplitCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Build the rectangle
        let mut pill = Rectangle::new()
            .set("width", 500)
            .set("height", 1000)
            .set("x", 0)
            .set("y", 0)
            .set("rx", 250)
            .set("ry", 250);

        // Add a rotation to the pill
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        pill = pill.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Generate the data for the split circle
        let data = match rotate_amount {
            0 => {
                // Pill is on the left, circle is on the bottom-left
                Data::new()
                    .move_to((0, 750))
                    .elliptical_arc_to((50, 50, 0, 0, 1, 500, 750))
            }
            90 => {
                // Pill is on the top, circle is on the top-left
                Data::new()
                    .move_to((250, 0))
                    .elliptical_arc_to((50, 50, 0, 0, 1, 250, 500))
            }
            180 => {
                // Pill is on the right, circle is on the top-right
                Data::new()
                    .move_to((500, 250))
                    .elliptical_arc_to((50, 50, 0, 0, 0, 1000, 250))
            }
            270 => {
                // Pill is on the bottom, circle is on the bottom-right
                Data::new()
                    .move_to((750, 500))
                    .elliptical_arc_to((50, 50, 0, 0, 0, 750, 1000))
            }
            _ => panic!("Not a valid rotation"),
        };

        // Generate the paths for the circle
        let mut circle = Path::new().set("d", data);

        // Set the fill, which can be either solid or gradient
        if random.roll::<u8>(100) < 80 {
            // Pick a solid color
            let (color_pill, color_circle) = if base_color.is_some() {
                // Use the base color and derive something similar
                let color_pill = base_color.unwrap().derive_similar_color(random);
                (
                    color_pill.as_string(),
                    HSL {
                        lightness: color_pill.lightness - 10,
                        ..color_pill
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

                let color_pill = HSL::new_random(random, color_mode, 100);

                (
                    color_pill.as_string(),
                    HSL {
                        lightness: color_pill.lightness - 10,
                        ..color_pill
                    }
                    .as_string(),
                )
            };

            pill = pill.set("fill", color_pill);
            circle = circle.set("fill", color_circle);

            vec![pill.into(), circle.into()]
        } else {
            // Get a gradient definition
            let ((gradient1, gradient1_name), (gradient2, gradient2_name)) = if base_color.is_some()
            {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);
                let color3 = base_color.unwrap().derive_similar_color(random);
                let color4 = base_color.unwrap().derive_similar_color(random);

                (
                    gradient_definition(random, Some(45), color1, color2),
                    gradient_definition(random, Some(45), color3, color4),
                )
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                (
                    random_gradient_definition(random, Some(45), color_mode, 100),
                    random_gradient_definition(random, Some(45), color_mode, 100),
                )
            };

            pill = pill.set("fill", format!("url(#{gradient1_name})"));
            circle = circle.set("fill", format!("url(#{gradient2_name})"));

            vec![
                gradient1.into(),
                gradient2.into(),
                pill.into(),
                circle.into(),
            ]
        }
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![
            // Small elements don't combine well with this element
            small_elements::small_element_arch::SmallElementArch.type_id(),
            small_elements::small_element_arch_shadow::SmallElementArchShadow.type_id(),
            small_elements::small_element_cross::SmallElementCross.type_id(),
            small_elements::small_element_cube::SmallElementCube.type_id(),
            small_elements::small_element_flower::SmallElementFlower.type_id(),
            small_elements::small_element_flower_shadow::SmallElementFlowerShadow.type_id(),
            small_elements::small_element_four_circles::SmallElementFourCircles.type_id(),
            small_elements::small_element_circle::SmallElementCircle.type_id(),
            small_elements::small_element_square::SmallElementSquare.type_id(),
            small_elements::small_element_split_circle_opposite::SmallElementSplitCircleOpposite
                .type_id(),
            small_elements::small_element_split_circle::SmallElementSplitCircle.type_id(),
            small_elements::small_element_star::SmallElementStar.type_id(),
            small_elements::small_element_star_shadow::SmallElementStarShadow.type_id(),
            small_elements::small_element_triangle::SmallElementTriangle.type_id(),
            small_elements::small_element_pill::SmallElementPill.type_id(),
            small_elements::small_element_pill_shadow::SmallElementPillShadow.type_id(),
            small_elements::small_element_straight_split_square::SmallElementStraightSplitSquare
                .type_id(),
            small_elements::small_element_stacked_pills::SmallElementStackedPills.type_id(),
            small_elements::small_element_diagonal_split_square::SmallElementDiagonalSplitSquare
                .type_id(),
            small_elements::small_element_octagon::SmallElementOctagon.type_id(),
            small_elements::small_element_octagon_shadow::SmallElementOctagonShadow.type_id(),
            small_elements::small_element_ball::SmallElementBall.type_id(),
            small_elements::small_element_double_circle::SmallElementDoubleCircle.type_id(),
            small_elements::small_element_double_diamond::SmallElementDoubleDiamond.type_id(),
            small_elements::small_element_cross_shadow::SmallElementCrossShadow.type_id(),
            small_elements::small_element_circle_shadow::SmallElementCircleShadow.type_id(),
            small_elements::small_element_square_shadow::SmallElementSquareShadow.type_id(),
            overlays::overlay_triangle::OverlayTriangle.type_id(),
            overlays::overlay_half_circle::OverlayHalfCircle.type_id(),
        ]
    }
}
