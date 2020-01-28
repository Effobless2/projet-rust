pub mod pixel {

    #[derive(Copy, Clone)]
    pub struct Pixel {
        r: u8,
        g: u8,
        b: u8
    }

    impl Pixel{
        pub fn new(red: u8, green: u8, blue: u8) -> Pixel{
            Pixel{r: red, g: green, b: blue}
        }
        
        pub fn red(self) -> u8 {
            self.r
        }
        
        pub fn blue(self) -> u8 {
            self.b
        }
        
        pub fn green(self) -> u8 {
            self.g
        }
        
        pub fn display(self) -> String {
            let mut result = String::new();
            result.push_str(&self.red().to_string());
            result.push(',');
            result.push_str(&self.green().to_string());
            result.push(',');
            result.push_str(&self.blue().to_string());
            return result;
        }
        
        pub fn inverse(&mut self){
            self.r = 255 - self.r;
            self.g = 255 - self.g;
            self.b = 255 - self.b;
        }
    }

    impl PartialEq for Pixel{
        fn eq(&self, other: &Self) -> bool{
            self.r == other.red()   && 
            self.g == other.green() && 
            self.b == other.blue()
        }
    }

    pub fn greyscale(pixel: Pixel) -> Pixel{
        let x = pixel.red() + pixel.green() + pixel.blue() / 3;
        Pixel::new(x, x, x)
    }
}

#[cfg(test)]
mod pixel_tests {
    pub use super::pixel as pixel_mod;

    #[test]
    fn pixel_init(){
        let p = pixel_mod::Pixel::new(1, 2, 5);
        assert_eq!(p.red(), 1);
        assert_eq!(p.green(), 2);
        assert_eq!(p.blue(), 5);
    }

    #[test]
    fn pixel_display(){
        let p = pixel_mod::Pixel::new(1, 2, 5);
        let x = p.display();
        assert_eq!("1,2,5", x);
    }

    #[test]
    fn pixel_inverse(){
        let mut p = pixel_mod::Pixel::new(55, 78, 10);
        p.inverse();
        assert_eq!(p.red(), 200);
        assert_eq!(p.green(), 177);
        assert_eq!(p.blue(), 245);
    }

    #[test]
    fn pixel_partial_eq(){
        let mut p1 = pixel_mod::Pixel::new(55, 78, 10);
        let p2 = pixel_mod::Pixel::new(55, 78, 10);
        assert_eq!(p1 == p2, true);
        assert_eq!(p1 != p2, false);
        p1.inverse();
        assert_eq!(p1 == p2, false);
        assert_eq!(p1 != p2, true);
    }

    #[test]
    fn pixel_clone(){
        let p = pixel_mod::Pixel::new(10, 15, 12);
        let mut pc = p.clone();
        assert_eq!(p == pc, true);
        pc.inverse();
        assert_eq!(p == pc, false);
    }

    #[test]
    fn pixel_copy(){
        let p = pixel_mod::Pixel::new(10, 15, 48);
        let mut pc = p;
        assert_eq!(p == pc, true);
        pc.inverse();
        assert_eq!(p != pc, true);
    }
/*
    #[test]
    fn imageTest(){
        let mut x = Vec::new();
        x.push(pixel_mod::Pixel::new(8, 10, 15));
        let image = pixel_mod::Image{
            height: 5,
            width: 10,
            pixels: x
        };
        assert_eq!(image.height, 5);
        assert_eq!(image.width, 10);
        assert_eq!(image.pixels.len(), 1);
    }*/
}