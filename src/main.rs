//! SPI example with the ST7789 using the ESP-RUST-BOARD
//!
//! Folowing pins are used:
//! RST       GPIO3
//! DC        GPIO4
//! BACKLIGHT GPIO5
//! SCLK      GPIO6
//! SDA       GPIO7
//!
//! Depending on your target and the board you are using you have to change the pins.
//!
//! For this example you need to hook up an ST7789 SPI display.
//! The display will display an image on ferris the crab on a black background.

use std::thread;
use std::time::Duration;

use embedded_hal::spi::MODE_3;

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::*;
use esp_idf_hal::units::FromValueType;

use display_interface_spi::SPIInterface;
use ili9341::{Ili9341, Orientation};

use embedded_graphics::image::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

// use mipidsi::{Builder, Orientation};

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take()?;
    let spi = peripherals.spi2;

    let rst = PinDriver::output(peripherals.pins.gpio33)?;
    let dc = PinDriver::output(peripherals.pins.gpio27)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio32)?;
    let sclk = peripherals.pins.gpio18;
    let sda = peripherals.pins.gpio23;
    // let sdi = peripherals.pins.gpio8;
    let cs = peripherals.pins.gpio14;

    let mut delay = Ets;

    // configuring the spi interface, note that in order for the ST7789 to work, the data_mode needs to be set to MODE_3
    let config = config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(MODE_3);

    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        sda,
        Option::<Gpio19>::None,
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    )?;

    // display interface abstraction from SPI and DC
    let di = SPIInterface::new(device, dc);

    log::info!("Set ILI9341");
    let mut display = match Ili9341::new(
        di,
        rst,
        &mut Ets,
        Orientation::Landscape,
        ili9341::DisplaySize320x480,
    ) {
        Ok(d) => d,
        Err(err) => {
            log::error!("new Ili9341 failed: {:?}", err);
            return Ok(());
        }
    };

    log::info!("Clear ILI9341");
    display.clear_screen(0x5555).expect("ok");

    // turn on the backlight
    backlight.set_high()?;

    let raw_image_data = ImageRawLE::new(include_bytes!("../ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    // draw image on black background
    ferris.draw(&mut display).unwrap();

    println!("Image printed!");

    

    loop {
        thread::sleep(Duration::from_millis(1000));


    }
}