use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{path::Data, Element, Path};

pub struct OverlayHalfCircle;

impl Layer for OverlayHalfCircle {
    fn generate(&self, random: &mut Random, _base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the half circle
        let data = Data::new()
            .move_to((0, 0))
            .elliptical_arc_to((45, 45, 0, 0, 1, 0, 1000));

        let mut path = Path::new().set("d", data);

        // Add rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        path = path.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Set the gradient
        let (gradient, gradient_name) = gradient_definition(
            random,
            None,
            HSL {
                // Opacity = 0
                hue: 0,
                saturation: 100,
                lightness: 100,
                opacity: 0,
            },
            HSL {
                // Opacity = 0
                hue: 0,
                saturation: 100,
                lightness: 100,
                opacity: 100,
            },
        );

        path = path.set("fill", format!("url(#{gradient_name})"));

        // Add the gradient and overlay to the elements vector
        elements.extend(vec![gradient.into(), path.into()]);

        // Return the elements
        elements
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
}
