pub mod pixel;
pub mod image;

pub use crate::image::image as image_mod;

pub use crate::pixel::pixel as pixelModule;
use std::path::Path;
use std::ffi::CStr;
use std::os::raw::c_char;


#[no_mangle]
pub extern fn dummy()->u8 {
    return 42;
}

#[no_mangle]
pub extern fn generate_invert_ppm_file(from: *const c_char, to: *const c_char){
    let s_from = unsafe { CStr::from_ptr(from) }; 
    let s_to = unsafe { CStr::from_ptr(to) }; 
    let image = image_mod::Image::new_with_file(Path::new(s_from.to_str().unwrap()));
    let invert = image_mod::invert(image);
    invert.save(Path::new(s_to.to_str().unwrap()));
}

#[no_mangle]
pub extern fn generate_grayscale_ppm_file(from: *const c_char, to: *const c_char){
    
    let s_from = unsafe { CStr::from_ptr(from) }; 
    let s_to = unsafe { CStr::from_ptr(to) }; 
    let image = image_mod::Image::new_with_file(Path::new(s_from.to_str().unwrap()));
    let grayscale = image_mod::grayscale(image);
    grayscale.save(Path::new(s_to.to_str().unwrap()));
}

#[no_mangle]
pub extern fn copy_ppm_file(from: *const c_char, to: *const c_char){
    let s_from = unsafe { CStr::from_ptr(from) }; 
    let s_to = unsafe { CStr::from_ptr(to) }; 
    let image = image_mod::Image::new_with_file(Path::new(s_from.to_str().unwrap()));
    image.save(Path::new(s_to.to_str().unwrap()));
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