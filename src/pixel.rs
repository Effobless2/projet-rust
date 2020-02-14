pub mod pixel {

    #[derive(Copy, Clone)]
    pub struct Pixel {
        r: u8,
        g: u8,
        b: u8
    }

    impl Pixel{
        /// Constructor Pixel
        /// # Arguments
        /// * `red` - A u8 (red value of the pixel)
        /// * `green` - A u8 (green value of the pixel)
        /// * `blue` - A u8 (blue value of the pixel)
        pub fn new(red: u8, green: u8, blue: u8) -> Pixel{
            Pixel{r: red, g: green, b: blue}
        }
        
        /// Get red to pixel
        /// # Return 
        /// * red pixel -> u8
        pub fn red(self) -> u8 {
            self.r
        }
        
        /// Get green to pixel
        /// # Return 
        /// * green pixel -> u8
        pub fn green(self) -> u8 {
            self.g
        }
        
        /// Get blue to pixel
        /// # Return 
        /// * blue pixel -> u8
        pub fn blue(self) -> u8 {
            self.b
        }
        
        /// Returns a string which describes the pixel
        /// # Returns
        /// * String representation of a pixel
        /// # Example
        /// a pixel p1 (red = 5, green = 8, blue = 255)
        ///     returns (5,8,255)
        pub fn display(self) -> String {
            let mut result = String::new();
            result.push_str(&self.red().to_string());
            result.push(',');
            result.push_str(&self.green().to_string());
            result.push(',');
            result.push_str(&self.blue().to_string());
            return result;
        }
        
        /// Reverse pixel values
        /// 
        /// # Example
        /// a pixel (r = 80, g = 100, b = 80)
        /// r = 255 - 80 = 175
        /// g = 255 - 100 = 155
        /// b = 255 - 80 = 175
        /// pixel = (r = 175, g = 155, b = 175)
        pub fn inverse(&mut self){
            self.r = 255 - self.r;
            self.g = 255 - self.g;
            self.b = 255 - self.b;
        }
    }

    /// Overriding the eq function (==)
    /// To compare two Pixels by their colors
    /// # Arguments
    /// * `other` - pixels which will be compared to the current one
    /// # Example
    /// two pixels p1 (2,5,255) and p2 (2,5,255)
    ///     p1 == p2 returns true
    /// two pixels p1 (5,5,255) and p2 (2,5,255)
    ///     p1 == p2 returns false
    impl PartialEq for Pixel{
        fn eq(&self, other: &Self) -> bool{
            self.r == other.red()   && 
            self.g == other.green() && 
            self.b == other.blue()
        }
    }

    /// grayscaled pixel
    /// 
    /// # Arguments 
    /// * `pixel` - A pixel 
    /// 
    /// # Return 
    /// * grayscaled pixel  -> pixel
    /// 
    /// # Example
    /// a pixel (r = 80, g = 100, b = 80)
    /// r + g + b / 3 = 80 + 100 + 80 / 3 = 260 / 3 = 86
    /// return pixel(r = 86, g = 86, b = 86)
    pub fn grayscale(pixel: Pixel) -> Pixel{
        let x = ((pixel.red() as u32 + pixel.green() as u32 + pixel.blue() as u32) / 3) as u8;
        Pixel::new(x, x, x)
    }
}

/// Unit Tests for the Pixel structure
#[cfg(test)]
mod pixel_tests {
    pub use super::pixel as pixel_mod;


    /// Testing the Pixel Constructor
    #[test]
    fn pixel_init(){
        let p = pixel_mod::Pixel::new(1, 2, 5);
        assert_eq!(p.red(), 1);
        assert_eq!(p.green(), 2);
        assert_eq!(p.blue(), 5);
    }

    /// Testing the correct String representation of a Pixel
    #[test]
    fn pixel_display(){
        let p = pixel_mod::Pixel::new(1, 2, 5);
        let x = p.display();
        assert_eq!("1,2,5", x);
    }

    /// Testing the correct reversing of a pixel
    #[test]
    fn pixel_inverse(){
        let mut p = pixel_mod::Pixel::new(55, 78, 10);
        p.inverse();
        assert_eq!(p.red(), 200);
        assert_eq!(p.green(), 177);
        assert_eq!(p.blue(), 245);
    }

    /// Testing the correct overriding of eq function
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

    /// Testing the correct clonage of a pixel
    #[test]
    fn pixel_clone(){
        let p = pixel_mod::Pixel::new(10, 15, 12);
        let mut pc = p.clone();
        assert_eq!(p == pc, true);
        pc.inverse();
        assert_eq!(p == pc, false);
    }

    /// Testing the correct copy of a pixel
    #[test]
    fn pixel_copy(){
        let p = pixel_mod::Pixel::new(10, 15, 48);
        let mut pc = p;
        assert_eq!(p == pc, true);
        pc.inverse();
        assert_eq!(p != pc, true);
    }

    /// Testing that a pixel is well grayscaled
    #[test]
    fn pixel_grayscale(){
        let p = pixel_mod::Pixel::new(10, 52, 10);
        let gray = pixel_mod::grayscale(p);
        assert_eq!(gray.red(), gray.green());
        assert_eq!(gray.red(), gray.blue());
        assert_eq!(gray.green(), gray.blue());
        assert_eq!(gray.red(), 24);
    }

}