use palette_extract::{get_palette_rgb, Color};

fn main() {
    let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];

    let r = get_palette_rgb(&pixels);

    assert_eq!(r.len(), 1);
    assert_eq!(r[0], Color::new(252, 4, 4));
}
