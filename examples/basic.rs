use image::ImageBuffer;
use palette_extract::get_palette_rgb;

fn main() {
    let image_dimension = 16;
    let img_buffer = ImageBuffer::from_fn(image_dimension, image_dimension, |_, _| {
        image::Rgb([255_u8, 0_u8, 0_u8])
    });

    let img_bytes = img_buffer.as_raw();

    let color_palette = get_palette_rgb(&img_bytes);

    color_palette.iter().for_each(|x| println!("{:?}", x));
}
