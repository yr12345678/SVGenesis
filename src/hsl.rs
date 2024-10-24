use random::Random;

// A relatively simple implementation of HSL so we can randomly
// generate colors and have some proper influence on how they are
// generated, such as setting ranges for hue, saturation and lightness.
//
// Includes methods to take an HSL color and return additional colors,
// such as complementary colors or monochromatic variants.
//
// The implementation is not very sophisticated, but it serves our
// needs and prevents needing another dependency and cutting all float
// usage from that.

#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    Normal,
    Vibrant,
    Light,
    Tone,
}

#[derive(Debug, Clone, Copy)]
pub struct HSL {
    pub hue: i16,       // 0-360
    pub saturation: i8, // 0-100
    pub lightness: i8,  // 0-100
    pub opacity: i8,    // 0-100 - Can't use floats
}

impl HSL {
    /// Returns a new HSL struct with according to the provided values
    pub fn new(hue: i16, saturation: i8, lightness: i8, opacity: i8) -> Self {
        HSL {
            hue,
            saturation,
            lightness,
            opacity,
        }
    }

    /// Uses Random to generate a random color. If necessary, takes a desired color mode into account.
    pub fn new_random(random: &mut Random, color_mode: ColorMode, opacity: i8) -> Self {
        match color_mode {
            ColorMode::Normal => Self::new(
                random.in_range::<u16>(0, 360) as i16,
                random.in_range::<u8>(0, 100) as i8,
                random.in_range::<u8>(0, 100) as i8,
                opacity,
            ),
            ColorMode::Vibrant => Self::new(
                random.in_range::<u16>(0, 360) as i16,
                random.in_range::<u8>(90, 100) as i8,
                random.in_range::<u8>(50, 70) as i8,
                opacity,
            ),
            ColorMode::Light => Self::new(
                random.in_range::<u16>(0, 360) as i16,
                random.in_range::<u8>(80, 100) as i8,
                random.in_range::<u8>(70, 85) as i8,
                opacity,
            ),
            ColorMode::Tone => Self::new(
                random.in_range::<u16>(0, 360) as i16,
                random.in_range::<u8>(60, 70) as i8,
                random.in_range::<u8>(55, 65) as i8,
                opacity,
            ),
        }
    }

    // Helper method to normalize the hue to stay within 0 to 360
    fn normalize_hue(hue: i16) -> i16 {
        if hue >= 360 {
            hue - 360
        } else {
            hue
        }
    }

    /// Returns the other two triadic colors based on the provided color. Also returns the provided color.
    pub fn triadic_colors(&self) -> (Self, Self, Self) {
        let hue1 = Self::normalize_hue(self.hue + 120);
        let hue2 = Self::normalize_hue(self.hue + 240);

        (
            *self,
            Self::new(hue1, self.saturation, self.lightness, self.opacity),
            Self::new(hue2, self.saturation, self.lightness, self.opacity),
        )
    }

    /// Method to return analogous colors based on the provided color, meaning a 30 hue shift for each color.
    /// Returns three colors, including the provided color, because its position in the pallette depends on its value.
    pub fn analogous_colors(&self) -> (Self, Self, Self) {
        let (hue1, hue2, hue3) = if self.hue < 30 {
            // Base color is near 0, so increase hue for the other two variants
            (self.hue, self.hue + 30, self.hue + 60)
        } else if self.hue > 330 {
            // Base color is near 100, so decrease hue for the other two variants
            (self.hue - 60, self.hue - 30, self.hue)
        } else {
            // Base color is in the middle, adjust both directions
            (self.hue - 30, self.hue, self.hue + 30)
        };

        (
            Self::new(hue1, self.saturation, self.lightness, self.opacity),
            Self::new(hue2, self.saturation, self.lightness, self.opacity),
            Self::new(hue3, self.saturation, self.lightness, self.opacity),
        )
    }

    /// Returns the complementary color based on the provided color. Also returns the provided color.
    ///
    /// (Provided color, Complementary color)
    pub fn complementary_colors(&self) -> (Self, Self) {
        let complementary_hue = Self::normalize_hue(self.hue + 180);

        (
            *self,
            Self::new(
                complementary_hue,
                self.saturation,
                self.lightness,
                self.opacity,
            ),
        )
    }

    /// Method to return monochromatic colors based on the provided color, meaning a 10 lightness shift for each color.
    /// Returns three colors, including the provided color, because its position in the pallette depends on its value.
    /// The colors are always sorted from light to dark.
    pub fn monochromatic_colors(&self) -> (Self, Self, Self) {
        let (lightness1, lightness2, lightness3) = if self.lightness < 10 {
            // Base color is near 0, so increase lightness for the other two variants
            (self.lightness + 20, self.lightness + 10, self.lightness)
        } else if self.lightness > 90 {
            // Base color is near 100, so decrease lightness for the other two variants
            (self.lightness, self.lightness - 10, self.lightness - 20)
        } else {
            // Adjust both directions
            (self.lightness - 10, self.lightness, self.lightness + 10)
        };

        (
            Self::new(self.hue, self.saturation, lightness1, self.opacity),
            Self::new(self.hue, self.saturation, lightness2, self.opacity),
            Self::new(self.hue, self.saturation, lightness3, self.opacity),
        )
    }

    /// Returns the split-complementary colors based on the provided color. Also returns the provided color.
    pub fn split_complementary_colors(&self) -> (Self, Self, Self) {
        let hue1 = Self::normalize_hue(self.hue + 150);
        let hue2 = Self::normalize_hue(self.hue - 150);

        (
            *self,
            Self::new(hue1, self.saturation, self.lightness, self.opacity),
            Self::new(hue2, self.saturation, self.lightness, self.opacity),
        )
    }

    /// Returns triadic colors as strings in the following format:
    ///
    /// hsla(hue,saturation%,lightness%,opacity)
    pub fn triadic_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::triadic_colors(self);

        (color1.as_string(), color2.as_string(), color3.as_string())
    }

    /// Returns analogous colors as strings in the following format:
    ///
    /// hsla(hue,saturation%,lightness%,opacity)
    pub fn analogous_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::analogous_colors(self);

        (color1.as_string(), color2.as_string(), color3.as_string())
    }

    /// Returns complementary colors as strings in the following format:
    ///
    /// hsla(hue,saturation%,lightness%,opacity)
    pub fn complementary_colors_as_string(&self) -> (String, String) {
        let (color1, color2) = Self::complementary_colors(self);

        (color1.as_string(), color2.as_string())
    }

    /// Returns monochromatic colors as strings in the following format:
    ///
    /// hsla(hue,saturation%,lightness%,opacity)
    pub fn monochromatic_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::monochromatic_colors(self);

        (color1.as_string(), color2.as_string(), color3.as_string())
    }
    /// Returns split-complementary colors as strings in the following format:
    ///
    /// hsla(hue,saturation%,lightness%,opacity)
    pub fn split_complementary_colors_as_strings(&self) -> (String, String, String) {
        let (color1, color2, color3) = Self::split_complementary_colors(self);

        (color1.as_string(), color2.as_string(), color3.as_string())
    }

    /// Returns the HSL color formatted as a string fit for use in SVG code in the following format:
    ///
    /// hsla(hue,saturation%,lightness%,opacity)
    pub fn as_string(&self) -> String {
        // Can't use floats, so we create a string for the opacity
        let opacity_string = match self.opacity {
            100 => "1".to_string(),
            _ => format!("0.{:0>2}", self.opacity),
        };

        format!(
            "hsla({},{}%,{}%,{})",
            self.hue, self.saturation, self.lightness, opacity_string
        )
    }

    /// Derives a color close to this color
    ///
    /// Returns the derived color
    pub fn derive_similar_color(&self, random: &mut Random) -> HSL {
        // Pick a new hue
        let new_hue = if self.hue < 40 {
            self.hue + random.in_range::<u8>(20, 40) as i16
        } else if self.hue > 320 {
            self.hue - random.in_range::<u8>(20, 40) as i16
        } else {
            match random.next_bool() {
                true => self.hue + random.in_range::<u8>(20, 40) as i16,
                false => self.hue - random.in_range::<u8>(20, 40) as i16,
            }
        };

        // Pick a new saturation
        let new_saturation = if self.saturation < 30 {
            self.saturation + random.in_range::<u8>(15, 30) as i8
        } else if self.saturation > 70 {
            self.saturation - random.in_range::<u8>(15, 30) as i8
        } else {
            match random.next_bool() {
                true => self.saturation + random.in_range::<u8>(15, 30) as i8,
                false => self.saturation - random.in_range::<u8>(15, 30) as i8,
            }
        };

        // Pick a new lightness
        let new_lightness = if self.lightness < 30 {
            self.lightness + random.in_range::<u8>(15, 30) as i8
        } else if self.lightness > 70 {
            self.lightness - random.in_range::<u8>(15, 30) as i8
        } else {
            match random.next_bool() {
                true => self.lightness + random.in_range::<u8>(15, 30) as i8,
                false => self.lightness - random.in_range::<u8>(15, 30) as i8,
            }
        };

        // Return the new color
        Self::new(new_hue, new_saturation, new_lightness, self.opacity)
    }
}
