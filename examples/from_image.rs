use mmcq::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};

fn main() {
    
    let tree_star = "./examples/test_images/tree_star.jpg";
    let leaf = "./examples/test_images/leaf.jpg";
    let sunset = "./examples/test_images/sunset.jpg";

    let img = image::open(tree_star).unwrap();
    let pixels = img.as_bytes();

    let res = get_palette_with_options(
        &pixels,
        PixelEncoding::Rgb,
        Quality::new(1),
        MaxColors::new(6),
        PixelFilter::None,
    );

    res.iter().for_each(|x| println!("{:?}", x));
}
