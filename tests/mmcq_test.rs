#[cfg(test)]
fn create_image(img_size: u32, get_pixel: fn(u32, u32) -> (u8, u8, u8)) -> Vec<u8> {
    let mut pixels: Vec<u8> = Vec::with_capacity(img_size as usize * 3);
    for x in 0..img_size {
        for y in 0..img_size {
            let pixel = get_pixel(x, y);
            pixels.push(pixel.0);
            pixels.push(pixel.1);
            pixels.push(pixel.2);
        }
    }

    pixels
}

#[cfg(test)]
mod get_palette_with_options {
    use super::create_image;
    use mmcq::{get_palette_with_options, Color, MaxColors, PixelEncoding, PixelFilter, Quality};

    #[test]
    fn minimal_rgba() {
        let pixels = [
            100, 100, 100, 255, 10, 10, 10, 255, 100, 100, 100, 255, 20, 20, 20, 255,
        ];

        let r = get_palette_with_options(
            &pixels,
            PixelEncoding::Rgba,
            Quality::new(1),
            MaxColors::new(10),
            PixelFilter::default(),
        );

        assert_eq!(r.len(), 9);
        assert_eq!(r[0], Color::new(100, 100, 100));
    }

    #[test]
    fn minimal_rgb() {
        let pixels = [100, 100, 100, 10, 10, 10, 100, 100, 100, 20, 20, 20];

        let r = get_palette_with_options(
            &pixels,
            PixelEncoding::Rgb,
            Quality::new(1),
            MaxColors::new(10),
            PixelFilter::default(),
        );

        assert_eq!(r.len(), 9);
        assert_eq!(r[0], Color::new(100, 100, 100));
    }

    #[test]
    fn red_rgb() {
        let pixels = create_image(280, |_, _| (255, 0, 0));

        let r = get_palette_with_options(
            &pixels,
            PixelEncoding::Rgb,
            Quality::new(1),
            MaxColors::new(3),
            PixelFilter::default(),
        );

        assert_eq!(r[0], Color::new(252, 4, 4))
    }

    #[test]
    fn solids() {
        let pixels = create_image(512, |x, _| {
            if x < 128 {
                return (0, 0, 255);
            } else if x >= 128 && x < 480 {
                return (0, 255, 0);
            }
            return (255, 0, 0);
        });

        let r = get_palette_with_options(
            &pixels,
            PixelEncoding::Rgb,
            Quality::new(1),
            MaxColors::new(4),
            PixelFilter::default(),
        );

        assert_eq!(r[0], Color::new(4, 4, 252));
        assert_eq!(r[1], Color::new(252, 4, 4));
        assert_eq!(r[2], Color::new(4, 252, 4));
        assert_eq!(r[3], Color::new(64, 188, 132));
    }
}

#[cfg(test)]
mod get_palette_rgb {
    use super::create_image;
    use mmcq::{get_palette_rgb, Color};

    #[test]
    fn red() {
        let pixels = create_image(280, |_, _| (255, 0, 0));

        let result = get_palette_rgb(&pixels);

        assert_eq!(result[0], Color::new(252, 4, 4));
        assert_eq!(result.len(), 9);
    }
}
