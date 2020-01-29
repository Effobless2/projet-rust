mod pixel;
mod image;
use std::path::Path;

pub use crate::image::image as image_mod;

pub use crate::pixel::pixel as pixelModule;


#[no_mangle]
pub extern fn dummy()->u8 {
    print!("caca");
    let pixels = image_mod::Image::new(Path::new("./test.ppm"));
    return 42;
}

/*
#[no_mangle]
pub extern fn readppm(filename : String) -> imageMod::Image{
    let mut x = Vec::new();
    x.push(image::Pixel::new(8, 7, 7));
    return image::Image{
        height:8,
        width:8, 
        pixels: x
    };
}

#[no_mangle]
pub extern fn write_ppm(image : imageMod::Image){
}
    
*/