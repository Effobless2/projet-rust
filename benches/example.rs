#[macro_use]

extern crate bencher;
use bencher::Bencher;
use std::path::Path;

use big_project::image as image;
use big_project::pixel as pixel;

/// Testing loading image Performance
fn new_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::Image::new_with_file(Path::new("./test.ppm"))
    )
}

/// Testing performances of an image inversion
fn invert_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::invert(image::image::Image::new_with_file(Path::new("./test.ppm")))
    )
}


/// Testing performances of the grayscaling of an image
fn grayscale_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::grayscale(image::image::Image::new_with_file(Path::new("./test.ppm")))
    )
}

/// Testing performances of the save of an image
fn save_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::Image::new_with_file(Path::new("./test.ppm")).save(Path::new("./bench_result.ppm"))
    )
}

/// Testing performances during the creation of a pixel
fn new_pixel_bench(bench: &mut Bencher){
    bench.iter(|| pixel::pixel::Pixel::new(1,2,3)
    )
}

/// Testing performances of inversion of a pixel
fn inverse_pixel_bench(bench: &mut Bencher){
    let mut my_pixel = pixel::pixel::Pixel::new(1,2,3);
    bench.iter(|| my_pixel.inverse())
}

/// Testing performances of grayscaling of a pixel
fn grayscale_pixel_bench(bench: &mut Bencher){
    let my_pixel = pixel::pixel::Pixel::new(1,2,3);
    bench.iter(|| pixel::pixel::grayscale(my_pixel))
}

benchmark_group!(
    benches, 
    new_image_bench, 
    invert_image_bench, 
    grayscale_image_bench, 
    save_image_bench,
    new_pixel_bench,
    inverse_pixel_bench,
    grayscale_pixel_bench
);
benchmark_main!(benches);