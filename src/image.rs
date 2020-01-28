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

pub struct Image {
    pub height : u8,
    pub width : u8,
    pub pixels : Vec<Pixel>
}
