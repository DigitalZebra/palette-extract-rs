# rust-mmcq
A Rust implementation of [Leptonica's](http://www.leptonica.org/) **m**odified **m**edian **c**ut **q**uantization.

Easily extract the color palette from an image!

<img src="examples/test_images/zebra.jpg" alt="drawing" width="648"/>

![color1](https://via.placeholder.com/72/7E8565/7E8565?text=+)
![color2](https://via.placeholder.com/72/18272C/18272C?text=+)
![color3](https://via.placeholder.com/72/CECBD0/CECBD0?text=+)
![color4](https://via.placeholder.com/72/4B5B4A/4B5B4A?text=+)
![color5](https://via.placeholder.com/72/ADB1BA/ADB1BA?text=+)
![color6](https://via.placeholder.com/72/615A4C/615A4C?text=+)
![color7](https://via.placeholder.com/72/3E3A3D/3E3A3D?text=+)
![color8](https://via.placeholder.com/72/B2C5CC/B2C5CC?text=+)
![color9](https://via.placeholder.com/72/404B2B/404B2B?text=+)


# Credits
This Rust implementation of MMCQ is adapted from Kazuki Ohara's [ColorThiefSwift](https://github.com/yamoridon/ColorThiefSwift).

## Special thanks
- Sven Woltmann - for the [Java Implementation](https://github.com/SvenWoltmann/color-thief-java). ColorThiefSwift is a port of this.
- Lokesh Dhakar - for the [JavaScript version](https://github.com/lokesh/color-thief).
- And of course, [Dan Bloomberg](https://github.com/DanBloomberg), for the [original paper on MMCQ](http://leptonica.org/papers/mediancut.pdf), and the [reference implementation](https://github.com/DanBloomberg/leptonica/blob/master/src/colorquant2.c), found in [Leptonica](https://github.com/DanBloomberg/leptonica).

# Installation
To use, add the following to `Cargo.toml` under `[dependencies]`:
```toml
mmcq = "0.1.0"
```

# Usage
Using the library consists of calling `get_palette_rgb` or `get_palette_with_options` with a set of RGB or RGBA pixels represented as a `u8` slice. 

## Basic
A minimal example using 4 red pixels represented in RGB looks like this:
```rust
use mmcq::{get_palette_rgb, Color};

fn main() {
    let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];

    let r = get_palette_rgb(&pixels);

    assert_eq!(r.len(), 1);
    assert_eq!(r[0], Color::new(252, 4, 4));
}
```

## Image from file/somewhere else
Here's one way to extract the color palette of an image by leveraging the `image` crate to read and decode an image file (see full working example in `examples` directory):
```rust
    let image_path = "./path/to/image.jpg";

    // open and decode the image using the `image` crate
    let img = image::open(image_path).unwrap();

    // grab a reference to the underlying pixels/RGB buffer
    let pixels = img.as_bytes();

    // extract the color palette
    let palette = get_palette_rgb(&pixels);

    // output the extracted color palette
    palette.iter().for_each(|x| println!("{:?}", x));

```




More usage examples can be found in the `examples` directory!


# Background/Further reading

- The paper describing the MMCQ algorithm: http://leptonica.org/papers/mediancut.pdf

- A great post on different ways to extract color palettes:
[Color quantization](https://spin.atomicobject.com/2016/12/07/pixels-and-palettes-extracting-color-palettes-from-images/)