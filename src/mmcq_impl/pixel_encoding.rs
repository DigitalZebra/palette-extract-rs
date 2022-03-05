use super::types::Pixel;

#[derive(Copy, Clone)]
pub enum PixelEncoding {
    Rgb,
    Rgba,
}

impl PixelEncoding {
    pub fn stride(&self) -> u8 {
        match self {
            PixelEncoding::Rgb => 3,
            PixelEncoding::Rgba => 4,
        }
    }

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
