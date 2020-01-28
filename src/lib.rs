mod image;

#[no_mangle]
pub extern fn dummy()->u8 {
    return 4;
}
    
#[no_mangle]
pub extern fn readppm(filename : String) -> image::Image{
    let mut x = Vec::new();
    x.push(image::Pixel::new(8, 7, 7));
    return image::Image{
        height:8,
        width:8, 
        pixels: x
    };
}

#[no_mangle]
pub extern fn write_ppm(image : image::Image){
}

#[no_mangle]
pub extern fn greyscale(pixel: image::Pixel) -> image::Pixel{
    let x = pixel.red() + pixel.green() + pixel.blue() / 3;
    image::Pixel::new(x, x, x)
}
    
    
#[cfg(test)]
mod tests {
    use crate::image;

    #[test]
    fn pixelInit(){
        let p = image::Pixel::new(1, 2, 5);
        assert_eq!(p.red(), 1);
        assert_eq!(p.green(), 2);
        assert_eq!(p.blue(), 5);
    }

    #[test]
    fn pixelDisplay(){
        let p = image::Pixel::new(1, 2, 5);
        let x = p.display();
        assert_eq!("1,2,5", x);
    }

    #[test]
    fn pixelInverse(){
        let mut p = image::Pixel::new(55, 78, 10);
        p.inverse();
        assert_eq!(p.red(), 200);
        assert_eq!(p.green(), 177);
        assert_eq!(p.blue(), 245);
    }

    #[test]
    fn pixelPartialEq(){
        let mut p1 = image::Pixel::new(55, 78, 10);
        let p2 = image::Pixel::new(55, 78, 10);
        assert_eq!(p1 == p2, true);
        assert_eq!(p1 != p2, false);
        p1.inverse();
        assert_eq!(p1 == p2, false);
        assert_eq!(p1 != p2, true);
    }

    #[test]
    fn pixelClone(){
        let p = image::Pixel::new(10, 15, 12);
        let mut pc = p.clone();
        assert_eq!(p == pc, true);
        pc.inverse();
        assert_eq!(p == pc, false);
    }

    #[test]
    fn pixelCopy(){
        let p = image::Pixel::new(10, 15, 48);
        let mut pc = p;
        assert_eq!(p == pc, true);
        pc.inverse();
        assert_eq!(p != pc, true);
    }

    #[test]
    fn imageTest(){
        let mut x = Vec::new();
        x.push(image::Pixel::new(8, 10, 15));
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