use super::types::Pixel;

/// Enum representing a pixel encoding.
#[derive(Copy, Clone)]
pub enum PixelEncoding {
    /// RGB encoding
    Rgb,

    /// RGBA encoding
    Rgba,
}

impl PixelEncoding {
    /// Get the number of bytes that represent an entire pixel.
    pub fn stride(&self) -> u8 {
        match self {
            PixelEncoding::Rgb => 3,
            PixelEncoding::Rgba => 4,
        }
    }

    /// Extracts a pixel from a buffer, offset by a certain amount.
    /// 
    /// # Arguments
    /// - `buffer` - The buffer to extract a pixel from
    /// - `idx` - The offset to begin at
    /// 
    /// # Returns
    /// Returns an extracted pixel for this particular encoding.
    pub fn extract_pixel(&self, buffer: &[u8], idx: usize) -> Pixel {
        let stride = self.stride();
        let offset = idx * stride as usize;

        match self {
            PixelEncoding::Rgb => {
                Pixel::new_rgb(buffer[offset], buffer[offset + 1], buffer[offset + 2])
            }
            PixelEncoding::Rgba => Pixel::new_rgba(
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ),
        }
    }
}
