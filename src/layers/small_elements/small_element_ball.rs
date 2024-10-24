use crate::hsl::*;
use crate::layers::Layer;
use random::Random;
use svg::node::element::{Circle, Definitions, Element, RadialGradient, Stop};

pub struct SmallElementBall;

impl Layer for SmallElementBall {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(35, 100) * 2; // Always an even number

        let mut circle = Circle::new()
            .set("cx", 500)
            .set("cy", 500)
            .set("r", random_radius);

        // Pick a color
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

        // Set up the radial gradient
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

        vec![defs.into(), circle.into()]
    }
}
