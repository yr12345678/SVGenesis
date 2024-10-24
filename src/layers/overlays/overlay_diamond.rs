use crate::hsl::*;
use crate::layers::Layer;
use crate::utils::*;
use random::Random;
use svg::node::element::{Element, Polygon};

pub struct OverlayDiamond;

impl Layer for OverlayDiamond {
    fn generate(&self, random: &mut Random, _base_color: &Option<HSL>) -> Vec<Element> {
        // Generate the areas
        let mut diamond = Polygon::new().set("points", "0,500 500,0 1000,500 500,1000");

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
                        opacity: 0,
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
        let (gradient, gradient_name) = radial_gradient_definition(random, None, color1, color2);

        diamond = diamond.set("fill", format!("url(#{gradient_name})"));

        // Return the elements
        vec![gradient.into(), diamond.into()]
    }

    fn exclusions(&self) -> Vec<std::any::TypeId> {
        vec![]
    }
}
