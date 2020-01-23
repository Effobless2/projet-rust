pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
pub struct Image {
    pub height : u8,
    pub width : u8,
    pub pixels : [Pixel; 1]
}
