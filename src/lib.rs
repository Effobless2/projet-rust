//use std::fmt::Write;
use std::io::Write;
use std::io::prelude::*;

mod image;

#[no_mangle]
pub extern fn dummy()->u8 {
    return 4;
}

#[no_mangle]
pub extern fn readppm(filename : String) -> image::Image{
    let mut x = Vec::new();
    x.push(image::Pixel{r:8,g:7,b:7});
    return image::Image{
        height:8, 
        width:8, 
        pixels: x
    };
}

#[no_mangle]
pub extern fn write_ppm(image : image::Image){

}