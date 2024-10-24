use std::any::TypeId;

use crate::{hsl::*, layers::Layer};
use random::Random;
use svg::node::element::{
    Definitions, Filter, FilterEffectDropShadow, LinearGradient, RadialGradient, Stop,
};

/// Generates a gradient using randomness
pub fn random_gradient_definition(
    random: &mut Random,
    rotation: Option<u16>,
    color_mode: ColorMode,
    opacity: i8,
) -> (Definitions, String) {
    // Get a random base color
    let random_color = HSL::new_random(random, color_mode, opacity);

    // Generate our color set
    let (random_color1, random_color2) = match random.roll::<u8>(4) {
        0 => {
            let (color1, color2, _) = random_color.analogous_colors();
            (color1, color2)
        }
        1 => {
            let (color1, color2, _) = random_color.monochromatic_colors();
            (color1, color2)
        }
        2 => {
            let (color1, color2, _) = random_color.split_complementary_colors();
            (color1, color2)
        }
        3 => {
            let color2 = HSL::new_random(random, color_mode, opacity);
            (random_color, color2)
        }
        _ => panic!("Invalid color variant"),
    };

    // Generate the gradient with the random colors and return the result
    gradient_definition(random, rotation, random_color1, random_color2)
}

/// Generates a gradient using color input
pub fn gradient_definition(
    random: &mut Random,
    rotation: Option<u16>,
    color1: HSL,
    color2: HSL,
) -> (Definitions, String) {
    // Set up the gradient
    let gradient_name = format!("gr{}", random.in_range::<u16>(0, 65535));
    let mut gradient = LinearGradient::new()
        .set("id", gradient_name.clone())
        .add(
            Stop::new()
                .set("offset", "0%")
                .set("stop-color", color1.as_string()),
        )
        .add(
            Stop::new()
                .set("offset", "100%")
                .set("stop-color", color2.as_string()),
        );

    // Apply rotation if necessary
    if rotation.is_some() {
        let rotation = rotation.unwrap();
        gradient = gradient.set("gradientTransform", format!("rotate({rotation}, 0.5, 0.5)"));
    }

    // Put the gradient in a definition and return that with its name, which can be used to refer to it in a fill
    let defs = Definitions::new().add(gradient);

    (defs, gradient_name)
}

/// Generates a radial gradient using color input
pub fn radial_gradient_definition(
    random: &mut Random,
    rotation: Option<u16>,
    color1: HSL,
    color2: HSL,
) -> (Definitions, String) {
    // Set up the radial gradient
    let gradient_name = format!("gr{}", random.in_range::<u16>(0, 65535));
    let mut gradient = RadialGradient::new()
        .set("id", gradient_name.clone())
        .add(
            Stop::new()
                .set("offset", "0%")
                .set("stop-color", color1.as_string()),
        )
        .add(
            Stop::new()
                .set("offset", "100%")
                .set("stop-color", color2.as_string()),
        );

    // Apply rotation if necessary
    if rotation.is_some() {
        let rotation = rotation.unwrap();
        gradient = gradient.set("gradientTransform", format!("rotate({rotation}, 0.5, 0.5)"));
    }

    // Put the gradient in a definition and return that with its name, which can be used to refer to it in a fill
    let defs = Definitions::new().add(gradient);

    (defs, gradient_name)
}

/// Generates a drop-shadow definition
pub fn drop_shadow_definition(
    random: &mut Random,
    dx: i8,
    dy: i8,
    std_deviation: i8,
    flood_color: HSL,
    flood_opacity: u8, // 0-100
) -> (Definitions, String) {
    // Create filter
    let filter_name = format!("f{}", random.in_range::<u16>(0, 65535));
    let mut filter = Filter::new()
        .set("x", "-50%")
        .set("y", "-50%")
        .set("width", "250%")
        .set("height", "250%")
        .set("id", filter_name.clone());

    // Create drop-shadow
    let flood_opacity_string = match flood_opacity {
        100 => "1".to_string(),
        _ => format!("0.{:0>2}", flood_opacity),
    };

    let drop_shadow = FilterEffectDropShadow::new()
        .set("dx", dx)
        .set("dy", dy)
        .set("stdDeviation", std_deviation)
        .set("flood-color", flood_color.as_string())
        .set("flood-opacity", flood_opacity_string);

    filter = filter.add(drop_shadow);

    // Put the filter in a definition and return that with its name, which can be used to refer to it
    let defs = Definitions::new().add(filter);

    (defs, filter_name)
}

/// Picks a random layer based on the weights of the layers
///
/// Returns None if no layer could be picked (because allowed_layers was empty for example)
pub fn pick_random_layer(
    random: &mut Random,
    allowed_layers: Vec<(Box<dyn Layer>, u32)>,
) -> Option<Box<dyn Layer>> {
    if !allowed_layers.is_empty() {
        let total_weight: u32 = allowed_layers.iter().map(|(_, weight)| weight).sum();

        let mut roll = random.roll::<u32>(total_weight);

        for (layer, weight) in allowed_layers {
            if roll < weight {
                return Some(layer);
            } else {
                roll -= weight
            }
        }
    }

    None
}

/// Generates a vector of allowed layers based on the provided exclusions
pub fn exclude_layers(
    available_layers: Vec<(Box<dyn Layer>, u32)>,
    exclusions: &[TypeId],
) -> Vec<(Box<dyn Layer>, u32)> {
    available_layers
        .into_iter()
        .filter(|(layer, _)| !exclusions.contains(&layer.layer_type()))
        .collect()
}
