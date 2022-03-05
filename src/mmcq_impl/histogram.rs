use std::{cmp, rc::Rc};

use super::{
    config::{HISTOGRAM_SIZE, RIGHT_SHIFT},
    util::color_index_from,
    vbox::VBox,
    PixelEncoding,
};

pub fn create_histogram_and_vbox(
    pixels: &[u8],
    encoding: PixelEncoding,
    quality: u8,
    ignore_white: bool,
) -> VBox {
    let mut histogram: Vec<u32> = vec![0; HISTOGRAM_SIZE.into()];

    let mut r_min = u8::MAX;
    let mut r_max = u8::MIN;
    let mut g_min = u8::MAX;
    let mut g_max = u8::MIN;
    let mut b_min = u8::MAX;
    let mut b_max = u8::MIN;

    let pixel_count = pixels.len() / encoding.stride() as usize;

    let mut idx = 0;

    let quality_stride = quality as usize;

    while idx < pixel_count {
        let p = encoding.extract_pixel(pixels, idx);

        if !p.is_opaque() || (ignore_white && p.is_white()) {
            idx = idx.checked_add(quality_stride).unwrap(); // TODO: fix upwrap
            continue;
        }

        let shifted_r = p.r >> RIGHT_SHIFT;
        let shifted_g = p.g >> RIGHT_SHIFT;
        let shifted_b = p.b >> RIGHT_SHIFT;

        r_min = cmp::min(r_min, shifted_r);
        r_max = cmp::max(r_max, shifted_r);
        g_min = cmp::min(g_min, shifted_g);
        g_max = cmp::max(g_max, shifted_g);
        b_min = cmp::min(b_min, shifted_b);
        b_max = cmp::max(b_max, shifted_b);

        // increment histgram
        let index = color_index_from(shifted_r, shifted_g, shifted_b);
        let x = histogram[index as usize];
        histogram[index as usize] = x + 1;

        idx = idx.checked_add(quality_stride).unwrap(); // TODO: fix upwrap
    }

    VBox::new(r_min, r_max, g_min, g_max, b_min, b_max, Rc::new(histogram))
}
