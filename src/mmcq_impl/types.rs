use std::fmt::{Debug, Display};

/// A struct representing a color - the output of the palette extraction.
#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    /// The red color channel.
    pub r: u8,
    
    /// The green color channel.
    pub g: u8,

    /// The blue color channel.
    pub b: u8,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Color")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field(
                "hex",
                &format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b),
            )
            .finish()
    }
}

impl Color {

    /// Creates a new Color struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use palette_extract::Color;
    ///
    /// Color::new(255, 255, 255);
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

pub enum ColorChannel {
    R,
    G,
    B,
}

#[derive(Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b, a: 255 }
    }

    pub fn is_white(&self) -> bool {
        self.r > 250 && self.g > 250 && self.b > 250
    }

    pub fn is_opaque(&self) -> bool {
        self.a > 125
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.r, self.g, self.b, self.a)
    }
}
