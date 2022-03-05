use std::cmp;
use std::ops::Range;

use super::config::MULTIPLIER;
use super::util::color_index_from;

use super::types::{Color, ColorChannel};
use std::rc::Rc;

pub struct VBox {
    r_min: u8,
    r_max: u8,
    g_min: u8,
    g_max: u8,
    b_min: u8,
    b_max: u8,
    pub histogram: Rc<Vec<u32>>,
    pub volume: Option<u32>,
    pub count: Option<u32>,
    pub average: Option<Color>,
}

impl VBox {
    pub fn r_range(&self) -> Range<u8> {
        self.r_min..(self.r_max + 1)
    }

    pub fn g_range(&self) -> Range<u8> {
        self.g_min..(self.g_max + 1)
    }

    pub fn b_range(&self) -> Range<u8> {
        self.b_min..(self.b_max + 1)
    }

    pub fn get_r_min(&self) -> u8 {
        self.r_min
    }
    pub fn get_r_max(&self) -> u8 {
        self.r_max
    }
    pub fn get_g_min(&self) -> u8 {
        self.g_min
    }
    pub fn get_g_max(&self) -> u8 {
        self.g_max
    }
    pub fn get_b_min(&self) -> u8 {
        self.b_min
    }
    pub fn get_b_max(&self) -> u8 {
        self.b_max
    }

    pub fn set_min(&mut self, v: u8, channel: &ColorChannel) {
        match channel {
            ColorChannel::R => {
                self.r_min = v;
            }
            ColorChannel::G => {
                self.g_min = v;
            }
            ColorChannel::B => {
                self.b_min = v;
            }
        }

        self.compute_average();
        self.compute_count();
        self.compute_new_volume();
    }

    pub fn set_max(&mut self, v: u8, channel: &ColorChannel) {
        match channel {
            ColorChannel::R => {
                self.r_max = v;
            }
            ColorChannel::G => {
                self.g_max = v;
            }
            ColorChannel::B => {
                self.b_max = v;
            }
        }

        self.compute_average();
        self.compute_count();
        self.compute_new_volume();
    }

    pub fn new(
        r_min: u8,
        r_max: u8,
        g_min: u8,
        g_max: u8,
        b_min: u8,
        b_max: u8,
        histogram: Rc<Vec<u32>>,
    ) -> VBox {
        let mut n = VBox {
            r_min,
            r_max,
            g_min,
            g_max,
            b_min,
            b_max,
            histogram,
            volume: None,
            count: None,
            average: None,
        };

        n.compute_average();
        n.compute_count();
        n.compute_new_volume();

        n
    }

    pub fn new_from(other: &VBox) -> VBox {
        VBox {
            r_min: other.r_min,
            r_max: other.r_max,
            g_min: other.g_min,
            g_max: other.g_max,
            b_min: other.b_min,
            b_max: other.b_max,
            histogram: Rc::clone(&other.histogram), // TODO: fix this
            volume: other.volume,
            count: other.count,
            average: other.average,
        }
    }

    fn compute_count(&mut self) {
        let mut count: u32 = 0;
        for r in self.r_range() {
            for g in self.g_range() {
                for b in self.b_range() {
                    let index = color_index_from(r, g, b);
                    let value_at_index = self.histogram[index as usize];
                    count = count + value_at_index as u32;
                }
            }
        }

        self.count = Some(count);
    }

    pub fn get_count(&self) -> u32 {
        match self.count {
            Some(v) => v,
            None => {
                panic!("Count not calced?")
            }
        }
    }

    fn compute_new_volume(&mut self) {
        let new_volume = (self.r_max + 1 - self.r_min) as u32
            * (self.g_max + 1 - self.g_min) as u32
            * (self.b_max + 1 - self.b_min) as u32;
        self.volume = Some(new_volume);
    }

    pub fn get_volume(&self) -> u32 {
        match self.volume {
            Some(v) => v,
            None => {
                panic!("Volume not yet computed?")
            }
        }
    }

    fn compute_average(&mut self) {
        let mut ntot: u32 = 0;

        let mut r_sum = 0;
        let mut g_sum = 0;
        let mut b_sum = 0;

        for r in self.r_range() {
            for g in self.g_range() {
                for b in self.b_range() {
                    let index = color_index_from(r, g, b);
                    let hval = self.histogram[index as usize] as f32;
                    ntot += hval as u32;
                    r_sum += (hval * ((r as f32) + 0.5) * MULTIPLIER as f32) as u32;
                    g_sum += (hval * ((g as f32) + 0.5) * MULTIPLIER as f32) as u32;
                    b_sum += (hval * ((b as f32) + 0.5) * MULTIPLIER as f32) as u32;
                }
            }
        }

        let average = if ntot > 0 {
            let r = (r_sum / ntot) as u8;
            let g = (g_sum / ntot) as u8;
            let b = (b_sum / ntot) as u8;
            Color::new(r, g, b)
        } else {
            let r =
                (MULTIPLIER as f32 * (self.r_min + self.r_max + 1) as f32 / 2.0).min(255.0) as u8;
            let g =
                (MULTIPLIER as f32 * (self.g_min + self.g_max + 1) as f32 / 2.0).min(255.0) as u8;
            let b =
                (MULTIPLIER as f32 * (self.b_min + self.b_max + 1) as f32 / 2.0).min(255.0) as u8;
            Color::new(r, g, b)
        };

        self.average = Some(average);
    }

    pub fn get_average(&self) -> Color {
        match self.average {
            Some(v) => v,
            None => {
                panic!("Average not set!")
            }
        }
    }

    pub fn widest_color_channel(&self) -> ColorChannel {
        let r_width = self.r_max - self.r_min;
        let g_width = self.g_max - self.g_min;
        let b_width = self.b_max - self.b_min;

        let m = cmp::max(cmp::max(r_width, g_width), b_width);
        match m {
            i if i == r_width => ColorChannel::R,
            i if i == g_width => ColorChannel::G,
            i if i == b_width => ColorChannel::B,
            _ => {
                panic!("shouldn't happen")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mmcq_impl::histogram::create_histogram_and_vbox;
    use crate::mmcq_impl::types::ColorChannel;
    use crate::mmcq_impl::PixelEncoding;

    fn create_pixels(dim: u32) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![];
        for _ in 0..=(dim * dim) {
            pixels.push(255); // r
            pixels.push(0); // g
            pixels.push(0); // b
        }

        pixels
    }

    #[test]
    fn gets_average() {
        let pixels = create_pixels(8);

        let vbox = create_histogram_and_vbox(&pixels, PixelEncoding::Rgb, 1, true);

        let color = vbox.get_average();
        assert_eq!(color.r, 252);
        assert_eq!(color.g, 4);
        assert_eq!(color.b, 4);
    }

    #[test]
    fn gets_average_min_max() {
        let pixels = create_pixels(8);

        let mut vbox = create_histogram_and_vbox(&pixels, PixelEncoding::Rgb, 1, true);

        vbox.set_max(31, &ColorChannel::R);
        vbox.set_min(32, &ColorChannel::R);

        let color = vbox.get_average();

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 4);
        assert_eq!(color.b, 4);
    }
}
