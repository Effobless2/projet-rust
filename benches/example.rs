#[macro_use]

extern crate bencher;
use bencher::Bencher;
use std::path::Path;

use big_project::image as image;
use big_project::pixel as pixel;

fn new_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::Image::new_with_file(Path::new("./test.ppm"))
    )
}

fn invert_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::invert(image::image::Image::new_with_file(Path::new("./test.ppm")))
    )
}

fn grayscale_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::grayscale(image::image::Image::new_with_file(Path::new("./test.ppm")))
    )
}

fn save_image_bench(bench: &mut Bencher) {
    bench.iter(|| image::image::Image::new_with_file(Path::new("./test.ppm")).save(Path::new("./bench_result.ppm"))
    )
}

fn new_pixel_bench(bench: &mut Bencher){
    bench.iter(|| pixel::pixel::Pixel::new(1,2,3)
    )
}

fn inverse_pixel_bench(bench: &mut Bencher){
    let mut my_pixel = pixel::pixel::Pixel::new(1,2,3);
    bench.iter(|| my_pixel.inverse())
}

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