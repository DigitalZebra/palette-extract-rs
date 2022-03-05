mod mmcq_impl;

use mmcq_impl::extract_colors;
pub use mmcq_impl::{Color, PixelEncoding};

pub struct Quality(u8);

impl Quality {
    pub fn new(quality: u8) -> Quality {
        Quality(quality)
    }
}

impl Default for Quality {
    fn default() -> Self {
        Quality(5)
    }
}

pub struct MaxColors(u8);

impl MaxColors {
    pub fn new(quality: u8) -> MaxColors {
        MaxColors(quality)
    }
}

impl Default for MaxColors {
    fn default() -> Self {
        MaxColors(10)
    }
}

#[derive(Eq, PartialEq)]
pub enum PixelFilter {
    None,
    White,
}

impl Default for PixelFilter {
    fn default() -> Self {
        PixelFilter::White
    }
}

pub fn get_palette_with_options(
    pixels: &[u8],
    encoding: PixelEncoding,
    quality: Quality,
    max_colors: MaxColors,
    pixel_filter: PixelFilter,
) -> Vec<Color> {
    let result = extract_colors(
        pixels,
        encoding,
        quality.0,
        max_colors.0,
        pixel_filter == PixelFilter::White,
    );

    result
}

pub fn get_palette_rgb(pixels: &[u8]) -> Vec<Color> {
    get_palette_with_options(
        pixels,
        PixelEncoding::Rgb,
        Quality::default(),
        MaxColors::default(),
        PixelFilter::White,
    )
}
