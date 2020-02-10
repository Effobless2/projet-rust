use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

use crate::lib::image;

pub fn new_with_file_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| image::Image::new_with_file(Path::new("./mandelbrot.ppm"))));
}
criterion_group!(benches, 
    new_with_file_benchmark);
criterion_main!(benches);