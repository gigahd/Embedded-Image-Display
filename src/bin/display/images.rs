use smart_leds::RGB8;
use crate::display::image_display::Displayable;

pub enum ImageDataType {
    RGB565,
    RGB888,
}

pub struct Image<'ch, const WIDTH: usize, const HEIGHT: usize, const PIXEL_COUNT: usize> {
    image_bytes: &'ch[u8],
    image_data_type: ImageDataType,
}

impl<'ch, const WIDTH: usize, const HEIGHT: usize, const PIXEL_COUNT: usize> Image<'ch, WIDTH, HEIGHT, PIXEL_COUNT> {
    pub fn new(image_bytes: &'ch[u8], image_type: ImageDataType) -> Self {
        assert_eq!(WIDTH * HEIGHT, PIXEL_COUNT, "WIDTH * HEIGHT must equal PIXEL COUNT");
        match image_type {
            ImageDataType::RGB565 => {
                // expect 2 bytes per pixel
                assert_eq!(image_bytes.len(), PIXEL_COUNT * 2,
                           "RGB565 data length must be WIDTH * HEIGHT * 2");
            },
            ImageDataType::RGB888 => {
                // expect 3 bytes per pixel
                assert_eq!(image_bytes.len(), PIXEL_COUNT * 3,
                           "RGB888 data length must be WIDTH * HEIGHT * 3");
            }
        }
        Self { image_bytes, image_data_type: image_type }
    }
}

impl<'ch, const WIDTH: usize, const HEIGHT: usize, const PIXEL_COUNT: usize> Displayable<PIXEL_COUNT> for Image<'ch, WIDTH, HEIGHT, PIXEL_COUNT> {
    fn to_rgb8_colors(&self) -> [RGB8; PIXEL_COUNT] {
        match self.image_data_type {
            ImageDataType::RGB888 => {
                core::array::from_fn(|i| {
                    let off = i * 3;
                    RGB8 {
                        r: self.image_bytes[off],
                        g: self.image_bytes[off + 1],
                        b: self.image_bytes[off + 2],
                    }
                })
            }
            ImageDataType::RGB565 => {
                core::array::from_fn(|i| {
                    let off = i * 2;
                    // here we interpret two bytes as big-endian 16-bit RGB565:
                    let hi = self.image_bytes[off + 1] as u16;
                    let lo = self.image_bytes[off] as u16;
                    let pix = (hi << 8) | lo;

                    // extract 5/6/5
                    let r5 = ((pix >> 11) & 0x1f) as u8;
                    let g6 = ((pix >> 5) & 0x3f) as u8;
                    let b5 = (pix & 0x1f) as u8;

                    // expand to 8 bits (simple bit replication)
                    let r = (r5 << 3) | (r5 >> 2);
                    let g = (g6 << 2) | (g6 >> 4);
                    let b = (b5 << 3) | (b5 >> 2);

                    RGB8 { r, g, b }
                })
            }
        }
    }
}