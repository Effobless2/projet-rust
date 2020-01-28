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



#[cfg(test)]
mod tests {
    use crate::image;
    #[test]
    fn pixelTest(){
        let p = image::Pixel{r:1,g:2,b:5};
        assert_eq!(p.r, 1);
        assert_eq!(p.g, 2);
        assert_eq!(p.b, 5);
    }

    #[test]
    fn imageTest(){
        let mut x = Vec::new();
        x.push(image::Pixel{
            r: 8,
            g: 10,
            b: 15
        });
        let image = image::Image{
            height: 5,
            width: 10,
            pixels: x
        };
        assert_eq!(image.height, 5);
        assert_eq!(image.width, 10);
        assert_eq!(image.pixels.len(), 1);
    }
}