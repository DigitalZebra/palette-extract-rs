use image::ImageBuffer;
use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};

fn main() {
    let image_dimension = 16;
    let img_buffer = ImageBuffer::from_fn(image_dimension, image_dimension, |_, _| {
        image::Rgb([255_u8, 0_u8, 0_u8])
    });

    let img_bytes = img_buffer.as_raw();

    let color_palette = get_palette_with_options(
        &img_bytes,
        PixelEncoding::Rgb,
        Quality::new(1),
        MaxColors::new(4),
        PixelFilter::White,
    );

    color_palette.iter().for_each(|x| println!("{:?}", x));
}
