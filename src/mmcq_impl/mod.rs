mod config;
mod histogram;
mod pixel_encoding;
mod types;
mod util;
mod vbox;

pub use types::Color;

pub use pixel_encoding::PixelEncoding;

use std::cmp::{self, Ordering};

use histogram::create_histogram_and_vbox;

use config::{FRACTION_BY_POPULATION, MAX_ITERATIONS, VBOX_LENGTH};
use util::color_index_from;
use vbox::VBox;

use self::types::ColorChannel;


/*
 * TODO:
 *  0. Find/fix other bugs, create more test cases, specifically the air fryer test...?
 *  2. Lazily compute avg/volume again, as right now computing it more frequently than needed
 *  3. Find other optimizations
 *  4. Clean up APIs, follow Rust cargo guide
 *  5. Move in to own repo
 *  6. Publish!
 */
pub fn extract_colors(
    pixels: &[u8],
    encoding: PixelEncoding,
    quality: u8,
    max_colors: u8,
    ignore_white: bool,
) -> Vec<Color> {
    let vbox = create_histogram_and_vbox(
        pixels,
        encoding,
        quality,
        ignore_white,
    );

    // priority queue
    let mut pq = vec![vbox];

    // Round up to have the same behaviour as in JavaScript
    let target = (FRACTION_BY_POPULATION * max_colors as f32).ceil() as u32;

    iterate(&mut pq, sort_by_count, target);

    pq.sort_by(sort_by_product);

    let len_before = pq.len() as u32;

    iterate(&mut pq, sort_by_product, max_colors as u32 - len_before);

    pq.reverse();

    pq.iter().map(|v| v.get_average()).collect()
}

fn apply_median_cut(vbox: VBox) -> Vec<VBox> {
    if vbox.get_count() == 0 {
        return vec![];
    }

    // only one pixel, no split
    if vbox.get_count() == 1 {
        return vec![vbox];
    }

    let histogram = &vbox.histogram;

    // Find the partial sum arrays along the selected axis.
    let mut total: u32 = 0;
    let mut partial_sum: Vec<i32> = vec![-1; VBOX_LENGTH.into()]; // -1 = not set / 0 = 0
    let axis = vbox.widest_color_channel();

    match axis {
        ColorChannel::R => {
            for r in vbox.r_range() {
                let mut sum: u32 = 0;
                for g in vbox.g_range() {
                    for b in vbox.b_range() {
                        let index = color_index_from(r, g, b);
                        sum += histogram[index as usize];
                    }
                }
                total += sum;
                partial_sum[r as usize] = total as i32;
            }
        }
        ColorChannel::G => {
            for g in vbox.g_range() {
                let mut sum: u32 = 0;
                for r in vbox.r_range() {
                    for b in vbox.b_range() {
                        let index = color_index_from(r, g, b);
                        sum += histogram[index as usize];
                    }
                }
                total += sum;
                partial_sum[g as usize] = total as i32;
            }
        }
        ColorChannel::B => {
            for b in vbox.b_range() {
                let mut sum: u32 = 0;
                for r in vbox.r_range() {
                    for g in vbox.g_range() {
                        let index = color_index_from(r, g, b);
                        sum += histogram[index as usize];
                    }
                }
                total += sum;
                partial_sum[b as usize] = total as i32;
            }
        }
    }

    let mut look_ahead_sum: Vec<i32> = vec![-1; VBOX_LENGTH.into()]; // -1 = not set / 0 = 0
    for (i, sum) in partial_sum.iter().enumerate().filter(|(_, &sum)| sum != -1) {
        look_ahead_sum[i] = total as i32 - sum
    }

    return cut(axis, &vbox, &partial_sum, &look_ahead_sum, total);
}

fn cut(
    axis: ColorChannel,
    vbox: &VBox,
    partial_sum: &[i32],
    look_ahead_sum: &[i32],
    total: u32,
) -> Vec<VBox> {
    let vbox_min: i32;
    let vbox_max: i32;

    match axis {
        ColorChannel::R => {
            vbox_min = vbox.get_r_min().into();
            vbox_max = vbox.get_r_max().into();
        }
        ColorChannel::G => {
            vbox_min = vbox.get_g_min().into();
            vbox_max = vbox.get_g_max().into();
        }
        ColorChannel::B => {
            vbox_min = vbox.get_b_min().into();
            vbox_max = vbox.get_b_max().into();
        }
    }

    for l in (vbox_min..(vbox_max + 1)).filter(|&i| partial_sum[i as usize] > (total / 2) as i32) {
        let mut vbox1 = VBox::new_from(&vbox);
        let mut vbox2 = VBox::new_from(&vbox);

        let left = l - vbox_min;
        let right = vbox_max - l;

        let mut d2 = if left <= right {
            cmp::min(vbox_max - 1, l + right / 2)
        } else {
            // 2.0 and cast to int is necessary to have the same
            // behaviour as in JavaScript
            cmp::max(vbox_min, (((l - 1) as f32) - (left as f32 / 2.0)) as i32)
        };

        while d2 < 0 || partial_sum[d2 as usize] <= 0 {
            d2 += 1;
        }

        let mut count2: i32 = look_ahead_sum[d2 as usize];
        while count2 == 0 && d2 > 0 && partial_sum[(d2 - 1) as usize] > 0 {
            d2 -= 1;
            count2 = look_ahead_sum[d2 as usize];
        }

        vbox1.set_max(d2 as u8, &axis);
        vbox2.set_min(d2 as u8 + 1, &axis);

        return vec![vbox1, vbox2];
    }

    panic!("VBox can't be cut")
}

fn sort_by_count(l: &VBox, r: &VBox) -> Ordering {
    l.get_count().cmp(&r.get_count())
}

fn sort_by_product(a: &VBox, b: &VBox) -> Ordering {
    let a_count = a.get_count();
    let b_count = b.get_count();
    let a_volume = a.get_volume();
    let b_volume = b.get_volume();

    if a_count == b_count {
        // If count is 0 for both (or the same), sort by volume
        return a_volume.cmp(&b_volume);
    } else {
        // Otherwise sort by products
        let a_product = a_count as u64 * a_volume as u64;
        let b_product = b_count as u64 * b_volume as u64;
        return a_product.cmp(&b_product);
    }
}

fn iterate(queue: &mut Vec<VBox>, comp: fn(&VBox, &VBox) -> Ordering, target: u32) {
    let mut color = 1;

    for _ in 0..MAX_ITERATIONS {
        let last_item = match queue.last() {
            Some(v) => v,
            None => return,
        };

        if last_item.get_count() == 0 {
            queue.sort_by(comp);
            continue;
        }
        let vbox = queue.pop().unwrap();
        let mut new_boxes = apply_median_cut(vbox);
        queue.push(new_boxes.remove(0));
        if new_boxes.len() == 1 {
            queue.push(new_boxes.remove(0));
            color += 1
        }
        queue.sort_by(comp);

        if color >= target {
            return;
        }
    }
}
