use core::{array::IntoIter, u8};

use smart_leds::RGB8;


const MAX_BRIGHTNESS: u8 = u8::MAX;

pub struct Display<'ch, const LED_COUNT: usize> {
    displayable: &'ch dyn Displayable<LED_COUNT>,
    pub brightness: u8,
}

impl<'ch, const LED_COUNT: usize> Display<'ch, LED_COUNT> {
    pub fn new(displayable: &'ch dyn Displayable<LED_COUNT>) -> Self {
        Display { displayable, brightness: MAX_BRIGHTNESS}
    }

    fn set_brightness(&self, colors: &mut [RGB8; LED_COUNT]) {
        colors.iter_mut().for_each(|color| {
            color.r = ((color.r as f64 / u8::MAX as f64) * self.brightness as f64) as u8;
            color.g = ((color.g as f64 / u8::MAX as f64) * self.brightness as f64) as u8;
            color.b = ((color.b as f64 / u8::MAX as f64) * self.brightness as f64) as u8;
        });
    }

    pub fn iter_colors(&self) -> IntoIter<RGB8, LED_COUNT> {
        let mut colors = self.displayable.to_rgb8_colors();
        if self.brightness < MAX_BRIGHTNESS {
            self.set_brightness(&mut colors);
        }
        colors.into_iter()
    }


}

pub trait Displayable<const PIXEL_COUNT: usize> {
    fn to_rgb8_colors(&self) -> [RGB8; PIXEL_COUNT];
}