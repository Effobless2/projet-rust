pub mod image{
    use std::io::Read;
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;
    use std::sync::mpsc;
    use std::thread;
    use std::io::{BufReader};
    use std::io::BufRead;

    use crate::pixel::pixel;

    #[derive(Clone)]
    pub struct Image{
        pub height: usize,
        pub width: usize,
        pub pixel_encoding: usize,
        pub pixels: Vec<pixel::Pixel>
    }

    impl Image{
        pub fn new_with_file(filename: &Path) -> Image{
            let (transmitter, receiver) = mpsc::channel();
            let ffile = File::open(filename);
            let file = BufReader::new(ffile.unwrap());
            let reader = thread::spawn(move || -> (usize, usize, usize) {
                

                let mut height :usize = 0;
                let mut width :usize = 0;
                let mut dimensions_found : bool = false;

                let mut code_found : bool = false;
                let mut code :usize = 0;


                for line in file.lines() {
                    let line = line.unwrap();
                    let chars : Vec<char> = line.chars().collect();
                    if chars.len() > 0 && chars[0] >= '0' && chars[0] <= '9' {
                        let list: Vec<&str> = line.split(' ').collect();
                        if list.len() == 2 && !dimensions_found{
                            height = list[0].parse().unwrap();
                            width = list[1].parse().unwrap();
                            dimensions_found = true;
                        }
                        else if list.len() == 1 && !code_found { 
                            code = list[0].parse().unwrap();
                            code_found = true;
                        }
                        else {
                            for val in list {
                                let x : u8 = val.parse().unwrap();
                                transmitter.send(x);
                            }
                        }
                    }
                }
                return (height, width, code);
            });

            let mut r_received :bool = false;
            let mut r :u8 = 0;
            let mut g_received :bool = false;
            let mut g :u8 = 0;
            let mut pixels : Vec<pixel::Pixel> = Vec::new();

            for received in receiver {
                if !r_received {
                    r = received;
                    r_received = true;
                } else if !g_received {
                    g = received;
                    g_received = true;
                } else {
                    pixels.push(pixel::Pixel::new(r, g, received));
                    r_received = false;
                    g_received = false;
                }
            }
            
            let (h, w, code) = reader.join().unwrap();

            return Image{
                height: h,
                width: w,
                pixel_encoding : code,
                pixels: pixels
            };
        }

        pub fn save(self, filename: &Path) -> std::io::Result<()>{
            let mut file = File::create(filename)?;
            file.write("p3\n".as_bytes())?;
            file.write(self.height.to_string().as_bytes())?;
            file.write(" ".as_bytes())?;
            file.write(self.width.to_string().as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write(self.pixel_encoding.to_string().as_bytes())?;
            file.write("\n".as_bytes())?;

            for i in 0..self.pixels.len() {
                file.write(self.pixels[i].red().to_string().as_bytes())?;
                file.write(" ".as_bytes())?;
                file.write(self.pixels[i].green().to_string().as_bytes())?;
                file.write(" ".as_bytes())?;
                file.write(self.pixels[i].blue().to_string().as_bytes())?;
                file.write("\n".as_bytes())?;
            }


            return Ok(());
        }
    }

    pub fn invert(image :Image) -> Image{
        let mut pixels = image.pixels;
        for  i in 0..pixels.len() {
            pixels[i].inverse();
        }
        return Image{
            height: image.height,
            width: image.width,
            pixels: pixels,
            pixel_encoding: image.pixel_encoding
        };
    }

    pub fn grayscale(image : Image) -> Image{
        let mut pixels = image.pixels;
        for  i in 0..pixels.len() {
            pixels[i] = pixel::grayscale(pixels[i]);
        }
        return Image{
            height: image.height,
            width: image.width,
            pixels: pixels,
            pixel_encoding: image.pixel_encoding
        };
    }
}

#[cfg(test)]
mod image_test{
    use std::path::Path;

    pub use super::image as image_mod;
    use crate::pixel::pixel;

    #[test]
    fn image_load(){

        let mut pixels : Vec<pixel::Pixel> = Vec::new();
        pixels.push(pixel::Pixel::new(255, 0, 0));
        pixels.push(pixel::Pixel::new(0, 255, 0));
        pixels.push(pixel::Pixel::new(0, 0, 255));
        pixels.push(pixel::Pixel::new(255, 255, 0));
        pixels.push(pixel::Pixel::new(255, 255, 255));
        pixels.push(pixel::Pixel::new(0, 0, 0));
        
        let image = image_mod::Image::new_with_file(Path::new("./test.ppm"));
        assert_eq!(image.pixels.len(), 6);

        assert_eq!(image.height, 3);
        assert_eq!(image.width, 2);
        assert_eq!(image.pixel_encoding, 255);
        assert_eq!(image.pixels.len(), pixels.len());
        for i in 0..image.pixels.len() {
            assert_eq!(image.pixels[i] == pixels[i], true);
        }
    }

    #[test]
    fn image_save(){
        let image = image_mod::Image::new_with_file(Path::new("./test.ppm"));
        let _result = image.save(Path::new("./result.ppm"));
        assert_eq!(1,1);
    }

    #[test]
    fn image_invert(){
        let mut pixels : Vec<pixel::Pixel> = Vec::new();
        pixels.push(pixel::Pixel::new(0, 255, 255));
        pixels.push(pixel::Pixel::new(255, 0, 255));
        pixels.push(pixel::Pixel::new(255, 255, 0));
        pixels.push(pixel::Pixel::new(0, 0, 255));
        pixels.push(pixel::Pixel::new(0, 0, 0));
        pixels.push(pixel::Pixel::new(255, 255, 255));

        let image = image_mod::Image::new_with_file(Path::new("./test.ppm"));
        let inv = image_mod::invert(image);
        assert_eq!(inv.height, 3);
        assert_eq!(inv.width, 2);
        assert_eq!(inv.pixel_encoding, 255);
        assert_eq!(inv.pixels.len(), pixels.len());
        for i in 0..inv.pixels.len() {
            assert_eq!(inv.pixels[i] == pixels[i], true);
        }
        let _result = inv.save(Path::new("./resultI.ppm"));
    }

    #[test]
    fn image_grayscale(){
        let mut pixels : Vec<pixel::Pixel> = Vec::new();
        pixels.push(pixel::Pixel::new(85, 85, 85));
        pixels.push(pixel::Pixel::new(85, 85, 85));
        pixels.push(pixel::Pixel::new(85, 85, 85));
        pixels.push(pixel::Pixel::new(170, 170, 170));
        pixels.push(pixel::Pixel::new(255, 255, 255));
        pixels.push(pixel::Pixel::new(0, 0, 0));

        let image = image_mod::Image::new_with_file(Path::new("./test.ppm"));
        let gray = image_mod::grayscale(image);
        assert_eq!(gray.height, 3);
        assert_eq!(gray.width, 2);
        assert_eq!(gray.pixel_encoding, 255);
        assert_eq!(gray.pixels.len(), pixels.len());
        for i in 0..gray.pixels.len() {
            assert_eq!(gray.pixels[i] == pixels[i], true);
        }
        let _result = gray.save(Path::new("./resultG.ppm"));
    }
}