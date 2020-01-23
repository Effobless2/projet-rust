mod image;

#[no_mangle]
pub extern fn dummy()->u8 {
    let p = image::Image {height:98, width:4, pixels:[image::Pixel{r:1,g:58,b:89}]};
    return p.height;
}
