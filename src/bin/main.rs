#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

mod display;

use core::u32::MAX;

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::rng::Rng;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use smart_leds::{RGB8, SmartLedsWrite};
use crate::display::image_display::Display;
use crate::display::images::Image;

use {esp_backtrace as _, esp_println as _};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]

//const LED_BUFFER_SIZE: usize = 1;
const STRIP_LED_COUNT: usize = 21;
const CIRCLE_LED_COUNT: usize = 93;
const CIRCLE_OUTER_RING_COUNT: usize = 32;
const GRID_LED_COUNT: usize = 64;
const LED_BRIGHTNESS_LEVEL: u8 = 2;
const COUNT: usize = GRID_LED_COUNT;



fn cycle_color(color: &mut RGB8) {
    let temp = color.r;
    color.r = color.b;
    color.b = color.g;
    color.g = temp;
}

fn apply_color_to_strip(color_strip: &mut [RGB8; COUNT], color: RGB8) {
    color_strip.iter_mut().for_each(| strip_color | {
        strip_color.r = color.r;
        strip_color.g = color.g;
        strip_color.b = color.b;
    });
}

fn move_color_with_wrap_around(color_strip: &mut [RGB8; COUNT], color: & RGB8, color_index: usize, spread: usize) {
    color_strip.iter_mut().enumerate().for_each(| (index, strip_color) | {
        if index >= color_index - spread && index <= color_index + spread {
            strip_color.r = color.r;
            strip_color.g = color.g;
            strip_color.b = color.b;
        } else {
            strip_color.r = 0;
            strip_color.g = 0;
            strip_color.b = 0;
        }
    });
}

fn set_brightness(color: &mut RGB8, brightness: u8) {
    color.r = brightness;
    color.g = brightness;
    color.b = brightness;
}

fn get_random_range(rng: &Rng) -> f64 {
    let random = rng.random();
    random as f64 / MAX as f64
}

fn randomize_color(color: &mut RGB8, rng: &Rng) {
    color.r = (color.r as f64 * get_random_range(rng)) as u8;
    color.g = (color.g as f64 * get_random_range(rng)) as u8;
    color.b = (color.b as f64 * get_random_range(rng)) as u8;
}

#[main]
fn main() -> ! {
    // generator version: 1.2.0
    let bytes = include_bytes!("../../assets/output_test_565.raw");
    let image: Image<'_, 8, 8, 64> = Image::new(bytes, display::images::ImageDataType::RGB565);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut buffer_strip = smart_led_buffer!(COUNT);
    
    let mut led_strip = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO5, &mut buffer_strip);
    
    let mut display: Display<COUNT> = Display::new(&image);
    display.brightness = LED_BRIGHTNESS_LEVEL;
    loop {

        led_strip.write(display.iter_colors()).unwrap();
        
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }
}
