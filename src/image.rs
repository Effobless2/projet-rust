pub mod image{
    use std::io::Read;
    use std::fs::File;
    use std::path::Path;

    use crate::pixel::pixel;

    pub struct Image{
        height: usize,
        width: usize,
        pixels: Vec<pixel::Pixel>
    }

    impl Image{
        pub fn new(filename: &Path) -> u32{
            let file = File::open(filename);

            let mut height_chars : Vec<char> = Vec::new();
            let mut height_found : bool = false;

            let mut width_chars : Vec<char> = Vec::new();
            let mut width_found : bool = false;

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
                        if  current_char < '0' ||
                            current_char > '9' {
                            code_found = true;
                        }
                    }
                    else {/*
                        if  current_char >= '0' &&
                            current_char <= '9' {
                            current_pixel.push(current_char);
                        } else {
                            if current_pixel.len() > 0 {
                                //conversion en u8
                                let mut current_value : u8 = 0;
                                for i in 0..current_pixel.len() {
                                    if i == current_pixel.len() - 1 {
                                        current_value += current_pixel[i] as u8 - '0' as u8;
                                    } else {
                                        current_value += (current_pixel[i] as u8 - '0' as u8) * u8::pow(10, (current_pixel.len() - i - 1) as u32);
                                    }
                                }
                                image_values.push(current_value);
                            }
                        }*/
                    }
                }
                i+= 1;
            }
            let height: u32 = Image::convert_vec_to_u32(height_chars);
            let width : u32 = Image::convert_vec_to_u32(width_chars);

            return height;
        }

        fn convert_vec_to_u32(vector : Vec<char>) -> u32{
            let mut result : u32 = 0;
            for i in 0..vector.len() {
                if i == vector.len() - 1{
                    result += (vector[i] as u8 - '0' as u8) as u32;
                } else {
                    result += (vector[i] as u8 - '0' as u8) as u32 * u32::pow(10, (vector.len() - i - 1) as u32) as u32;
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

    #[test]
    fn image_load_test(){
        let height = image_mod::Image::new(Path::new("./test.ppm"));
        assert_eq!(height, 1);
    }
}