#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

mod display;

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use smart_leds::SmartLedsWrite;
use crate::display::image_display::Display;
use crate::display::images::{AnimatedImage, Image};

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
const LED_BRIGHTNESS_LEVEL: u8 = 100;
const COUNT: usize = GRID_LED_COUNT;

const GRID_WIDTH: usize = 8;
const GRID_HEIGHT: usize = 8;

const  GRID_COUNT: usize = GRID_WIDTH * GRID_HEIGHT;

#[main]
fn main() -> ! {
    // generator version: 1.2.0
    
    let bytes_one = include_bytes!("../../assets/output_test_1.raw");
    let image_one: Image<'_, GRID_WIDTH, GRID_HEIGHT, GRID_COUNT> = Image::new(bytes_one, display::images::ImageDataType::RGB565);
    let bytes_two = include_bytes!("../../assets/output_test_2.raw");
    let image_two: Image<'_, GRID_WIDTH, GRID_HEIGHT, GRID_COUNT> = Image::new(bytes_two, display::images::ImageDataType::RGB565);
    let bytes_three = include_bytes!("../../assets/output_test_3.raw");
    let image_three: Image<'_, GRID_WIDTH, GRID_HEIGHT, GRID_COUNT> = Image::new(bytes_three, display::images::ImageDataType::RGB565);
    let bytes_four = include_bytes!("../../assets/output_test_4.raw");
    let image_four: Image<'_, GRID_WIDTH, GRID_HEIGHT, GRID_COUNT> = Image::new(bytes_four, display::images::ImageDataType::RGB565);


    let animated_image = AnimatedImage::new([&image_one, &image_two, &image_three, &image_four]);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut buffer_strip = smart_led_buffer!(COUNT);
    
    let mut led_strip = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO5, &mut buffer_strip);
    
    let mut display: Display<COUNT> = Display::new(&animated_image);
    display.brightness = LED_BRIGHTNESS_LEVEL;
    loop {

        led_strip.write(display.iter_colors()).unwrap();
        
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(10) {}

        animated_image.update_image(0.01);
    }
}
