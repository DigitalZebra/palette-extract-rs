#![warn(missing_docs)]
//! A lib crate for extracting a color palette from an image represented as a `u8` slice.
//! 
//! Supports `RGB` and `RGBA`.
//! 
//! # Examples
//! See `examples` directory in code repo for a full set of functioning examples!
//! 
//! ## Basic
//! ```
//! use palette_extract::get_palette_rgb;
//! let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
//!
//! let palette = get_palette_rgb(&pixels);
//! ```
//! 
//! ## With options
//! 
//! ```
//! use palette_extract::{get_palette_with_options, Quality, MaxColors, PixelEncoding, PixelFilter};
//!
//! let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
//! 
//! let color_palette = get_palette_with_options(&pixels,
//!     PixelEncoding::Rgb,
//!     Quality::new(1),
//!     MaxColors::new(4),
//!     PixelFilter::White);
//! 
//! ```

mod mmcq_impl;

use mmcq_impl::extract_colors;
pub use mmcq_impl::{Color, PixelEncoding};

/// Represents the quality level used to extract the color palette. Defaults to 5.
pub struct Quality(u8);

impl Quality {
    /// Creates a new Quality struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use palette_extract::Quality;
    ///
    /// Quality::new(1);
    /// ```
    pub fn new(quality: u8) -> Quality {
        Quality(quality)
    }
}

impl Default for Quality {
    fn default() -> Self {
        Quality(5)
    }
}

/// Represents the max number of colors to extract from the image. Defaults to 10.
pub struct MaxColors(u8);

impl MaxColors {
    /// Creates a new ['MaxColors'](self::MaxColors) struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use palette_extract::MaxColors;
    ///
    /// MaxColors::new(5);
    /// ```
    pub fn new(max_colors: u8) -> MaxColors {
        MaxColors(max_colors)
    }
}

impl Default for MaxColors {
    fn default() -> Self {
        MaxColors(10)
    }
}

/// Represents a filter that can be applied to algorithm to filter out particular pixels.
#[derive(Eq, PartialEq)]
pub enum PixelFilter {
    /// Represents no filter. I.E. all colors/pixels will be considered.
    None,

    /// Represents a white pixel filter. All white pixels will be discarded for the purpose of extracting the palette from the image.
    White,
}

impl Default for PixelFilter {
    fn default() -> Self {
        PixelFilter::White
    }
}

/// Extracts a color palette from a slice of RGB color bytes represented with `u8`. Allows setting of various options.
///
/// # Arguments
/// - `pixels` - `u8` slice of pixels to extract the palette from.
/// - `encoding` - How the pixels are represented in `pixels` slice. RGB or RGBA are most common, depending on source image.
/// - `quality` - The number of pixels to consider when extracting the palette. A higher number will run quicker, but may be less accurate.
/// - `max_colors` - The max number of colors to extract from the image.
/// - `pixel_filter` - A filter applied to the pixels to exclude from considering. This can be useful if, for example, the subject of the image has a single color background (e.g. white) that shouldn't be considered in the palette.
/// 
/// # Examples
/// ```
/// use palette_extract::{get_palette_with_options, Quality, MaxColors, PixelEncoding, PixelFilter};
///
/// let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
/// 
/// let color_palette = get_palette_with_options(&pixels,
///     PixelEncoding::Rgb,
///     Quality::new(1),
///     MaxColors::new(4),
///     PixelFilter::White);
/// 
/// ```
/// 
/// # Panics
/// Panics if we are unable to perform an iteration of the algorithm.
///
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

/// Extracts a color palette from a slice of RGB color bytes represented with `u8`. Uses ['Quality'](Quality) of 5, ['MaxColors'](MaxColors) of 10, and ['PixelFilter::None'](PixelFilter::None).
///
/// Uses default options. See ['get_palette_with_options'](get_palette_with_options) to extract a color palette with
///
/// # Arguments
/// - `pixels` - `u8` slice of pixels to extract the palette from.
/// 
/// # Examples
///
/// ```
/// use palette_extract::get_palette_rgb;
/// let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
///
/// let palette = get_palette_rgb(&pixels);
/// ```
/// # Panics
/// Panics if we are unable to perform an iteration of the algorithm.
///
pub fn get_palette_rgb(pixels: &[u8]) -> Vec<Color> {
    get_palette_with_options(
        pixels,
        PixelEncoding::Rgb,
        Quality::default(),
        MaxColors::default(),
        PixelFilter::None,
    )
}
