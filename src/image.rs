pub mod image{
    use std::io::Read;
    use std::fs::File;
    use std::path::Path;

    use crate::pixel::pixel;

    pub struct Image{
        pub height: usize,
        pub width: usize,
        pub pixels: Vec<pixel::Pixel>
    }

    impl Image{
        pub fn new_with_file(filename: &Path) -> Image{
            let file = File::open(filename);

            let mut height_chars : Vec<char> = Vec::new();
            let mut height_found : bool = false;

            let mut width_chars : Vec<char> = Vec::new();
            let mut width_found : bool = false;

            let mut code : Vec<char> = Vec::new();
            let mut code_found : bool = false;


            let mut image_values : Vec<u8> = Vec::new();

            let mut current_pixel : Vec<char> = Vec::new();

            let mut i : usize = 0;
            for byte in file.unwrap().bytes() {
                if i > 3 {
                        let current_char : char = byte.unwrap() as char;
                    if !height_found { //
                        if  current_char >= '0' &&
                            current_char <= '9' {
                                height_chars.push(current_char);
                        } else if height_chars.len() > 0 {
                            height_found = true;
                        }
                    }
                    else if !width_found {
                        if  current_char >= '0' &&
                            current_char <= '9' {
                                width_chars.push(current_char);
                        } else if width_chars.len() > 0 {
                            width_found = true;
                        }
                    }
                    else if !code_found {
                        if  current_char >= '0' &&
                            current_char <= '9' {
                                code.push(current_char);
                        } else if code.len() > 0 {
                            code_found = true;
                        }
                    }
                    else {
                        if  current_char >= '0' &&
                            current_char <= '9' {
                            current_pixel.push(current_char);
                        } else if current_pixel.len() > 0 {
                            image_values.push(Image::convert_vec_to_u8(current_pixel));
                            current_pixel = Vec::new();
                        }
                    }
                }
                i+= 1;
            }
            image_values.push(Image::convert_vec_to_u8(current_pixel));


            let height: usize = Image::convert_vec_to_usize(height_chars);
            let width : usize = Image::convert_vec_to_usize(width_chars);

            let mut pixels : Vec<pixel::Pixel> = Vec::new();
            for i in 0..image_values.len() + 1{
                if i % 3 == 2 {
                    pixels.push(pixel::Pixel::new(image_values[i - 2], image_values[i - 1], image_values[i]))
                }
            }
            return Image{
                height: height,
                width: width,
                pixels: pixels
            };
        }

        fn convert_vec_to_usize(vector : Vec<char>) -> usize{
            let mut result : usize = 0;
            for i in 0..vector.len() {
                if i == vector.len() - 1{
                    result += (vector[i] as u8 - '0' as u8) as usize;
                } else {
                    result += (vector[i] as u8 - '0' as u8) as usize * usize::pow(10, (vector.len() - i - 1) as u32) as usize;
                }
            }
            return result;
        }

        fn convert_vec_to_u8(vector : Vec<char>) -> u8{
            let mut result : u8 = 0;
            for i in 0..vector.len() {
                if i == vector.len() - 1{
                    result += vector[i] as u8 - '0' as u8;
                } else {
                    result += (vector[i] as u8 - '0' as u8) * u32::pow(10, (vector.len() - i - 1) as u32) as u8;
                }
            }
            return result;
        }
    }
}

#[cfg(test)]
mod image_test{
    use std::path::Path;

    pub use super::image as image_mod;
    use crate::pixel::pixel;

    #[test]
    fn image_load_test(){

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
        assert_eq!(image.pixels.len(), pixels.len());
        for i in 0..image.pixels.len() {
            assert_eq!(image.pixels[i] == pixels[i], true);
        }
    }
}