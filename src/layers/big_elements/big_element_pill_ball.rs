use std::any::Any;

use crate::layers::{overlays, Layer};
use crate::utils::*;
use crate::{hsl::*, layers::small_elements};
use random::Random;
use svg::node::element::{Circle, Definitions, Element, RadialGradient, Rectangle, Stop};

pub struct BigElementPillBall;

impl Layer for BigElementPillBall {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Pick a rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        // Generate the pill
        let mut pill = Rectangle::new()
            .set("width", 500)
            .set("height", 1000)
            .set("x", 0)
            .set("y", 0)
            .set("rx", 250)
            .set("ry", 250)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Pick pill color
        if random.roll::<u8>(100) < 80 {
            // Solid color
            let color_pill = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
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

            pill = pill.set("fill", color_pill);

            elements.push(pill.into())
        } else {
            // Get a gradient definition and a color
            let (gradient_pill, gradient_pill_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

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

            pill = pill.set("fill", format!("url(#{gradient_pill_name})"));

            elements.extend(vec![gradient_pill.into(), pill.into()])
        }

        // Generate the ball
        let mut circle = Circle::new()
            .set("cx", 250)
            .set("cy", 750)
            .set("r", 250)
            .set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Pick a ball color
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

        // Set up the radial gradient for the ball
        let gradient_name = format!("gr{}", random.in_range::<u16>(0, 65535));
        let gradient_color1 = HSL {
            lightness: 95,
            ..color
        };
        let gradient_color2 = HSL {
            lightness: 70,
            ..color
        };

        let gradient = RadialGradient::new()
            .set("id", gradient_name.clone())
            .set("cx", "0.3")
            .set("cy", "0.3")
            .set("r", "0.7")
            .set(
                "gradientTransform",
                format!("rotate(-{rotate_amount}, 0.5, 0.5)"),
            ) // Gradient remains the same regardless of ball rotation
            .add(
                Stop::new()
                    .set("offset", "10%")
                    .set("stop-color", gradient_color1.as_string()),
            )
            .add(
                Stop::new()
                    .set("offset", "100%")
                    .set("stop-color", gradient_color2.as_string()),
            );

        let defs = Definitions::new().add(gradient);

        circle = circle.set("fill", format!("url(#{gradient_name})"));

        elements.extend(vec![defs.into(), circle.into()]);

        elements
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
