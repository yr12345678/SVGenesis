use crate::hsl::*;
use crate::layers::*;
use random::Random;
use svg::node::element::Definitions;
use svg::node::element::Element;
use svg::Document;
use svg::Node;

pub fn generate_nft_image_data(seed: &Vec<u8>) -> (String, Vec<String>) {
    // Instantiate the randomness
    let mut random = Random::new(seed);

    // Set up our stack of layers
    let mut layers: Vec<Box<dyn Layer>> = Vec::new();

    // Optionally pick a base color
    let base_color = if random.roll::<u8>(100) < 30 {
        let roll = random.roll::<u8>(100);
        let color_mode = if roll < 20 {
            ColorMode::Tone
        } else if roll < 50 {
            ColorMode::Light
        } else {
            ColorMode::Vibrant
        };

        Some(HSL::new_random(&mut random, color_mode, 100))
    } else {
        None
    };

    // Make sure we have at least 2 layers (background + ...)
    while layers.len() < 2 {
        // Start clean
        layers.clear();
        let mut exclusions = vec![];

        // Always add a background and add exclusions to the exclusions list
        let background = random_background(&mut random);
        exclusions.append(&mut background.exclusions());
        layers.push(background);

        // Potentially add a frame and add any exclusions to the exclusions list
        if random.roll::<u8>(100) < 10 && base_color.is_some() {
            if let Some(frame) = random_frame(&mut random, &exclusions) {
                exclusions.append(&mut frame.exclusions());
                layers.push(frame);
            }
        }

        // Potentially add a big element and add any exclusions to the exclusions list
        if random.next_bool() {
            if let Some(big_element) = random_big_element(&mut random, &exclusions) {
                exclusions.append(&mut big_element.exclusions());
                layers.push(big_element);
            }

            // Potentially add an overlay and add any exclusions to the exclusions list
            if random.roll::<u16>(100) < 5 {
                if let Some(overlay) = random_overlay(&mut random, &exclusions) {
                    exclusions.append(&mut overlay.exclusions());
                    layers.push(overlay);
                }
            }
        }

        // Potentially add a small element and add any exclusions to the exclusions list
        if random.next_bool() {
            if let Some(small_element) = random_small_element(&mut random, &exclusions) {
                exclusions.append(&mut small_element.exclusions());
                layers.push(small_element);
            }
        }
    }

    // Generate the SVG
    let (document, layer_names) = generate_svg(layers, &mut random, &base_color);

    (document.to_string(), layer_names)
}

fn generate_svg(
    layers: Vec<Box<dyn Layer>>,
    random: &mut Random,
    base_color: &Option<HSL>,
) -> (Document, Vec<String>) {
    // Set up the base Document
    let mut document = Document::new().set("viewBox", (0, 0, 1000, 1000));

    // Vectors we need
    let mut defs = Definitions::new();
    let mut layer_elements_to_add: Vec<Element> = vec![];
    let mut layer_names: Vec<String> = vec![];

    // Seperate definition and layer elements so we can add just a single Definitions node to the beginning of the document
    for layer in layers {
        // Generate all layer elements, including definitions
        let elements = layer.generate(random, base_color);

        // Go through all elements and add all definition elements to a larger vector
        for element in elements {
            match element.get_name().as_str() {
                "defs" => {
                    for def_element in element.get_children() {
                        defs.append(def_element.to_owned())
                    }
                }
                _ => {
                    layer_elements_to_add.push(element);
                }
            }
        }

        // Add the layer name to the layer name vector; we store this on the NFT data.
        layer_names.push(layer.layer_name());
    }

    // Add the definitions to the document
    if !defs.get_children().is_empty() {
        document.append(defs)
    };

    // Add the layer elements to the document
    for layer in layer_elements_to_add {
        document.append(layer);
    }

    (document, layer_names)
}
