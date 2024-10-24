use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct OverlayTriangle;

impl Layer for OverlayTriangle {
    fn generate(&self, random: &mut Random, _base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the areas
        let mut triangle = Polygon::new().set("points", "0,0 0,1000 500,500");

        // Add rotation
        let valid_rotate_amounts = [0, 90, 180, 270];
        let rotate_amount = valid_rotate_amounts
            .get(random.roll::<usize>(4))
            .expect("Did not find a valid rotation amount. This should never happen.");

        triangle = triangle.set("transform", format!("rotate({rotate_amount}, 500, 500)"));

        // Initalialize the elements vector
        let mut elements: Vec<Element> = vec![];

        // Pick a color
        let (color1, color2) = match random.roll::<u8>(2) {
            0 => {
                // White
                (
                    HSL {
                        hue: 0,
                        saturation: 100,
                        lightness: 100,
                        opacity: 0,
                    },
                    HSL {
                        hue: 0,
                        saturation: 100,
                        lightness: 100,
                        opacity: 100,
                    },
                )
            }
            1 => {
                // Black
                (
                    HSL {
                        hue: 0,
                        saturation: 100,
                        lightness: 0,
                        opacity: 10,
                    },
                    HSL {
                        hue: 0,
                        saturation: 100,
                        lightness: 0,
                        opacity: 80,
                    },
                )
            }
            _ => panic!("No matching overlay color type found!"),
        };

        // Set the gradient
        let (gradient, gradient_name) = gradient_definition(random, None, color1, color2);

        triangle = triangle.set("fill", format!("url(#{gradient_name})"));

        // Add triangle to vector of elements
        elements.extend(vec![gradient.into(), triangle.into()]);

        // Return the elements
        elements
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
}
