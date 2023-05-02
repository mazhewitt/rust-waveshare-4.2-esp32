// Import necessary libraries and modules
use anyhow;
use esp_idf_sys;

use embedded_graphics::mono_font::iso_8859_13::FONT_10X20;
use embedded_graphics::{
    pixelcolor::BinaryColor::On as Black, prelude::*, primitives::{Line, PrimitiveStyle},
    mono_font::{MonoTextStyle},
    text::Text,
};
use epd_waveshare::{epd4in2::*, graphics::VarDisplay};
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::spi::{config, Dma, SPI2, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::prelude::*;


fn main() -> anyhow::Result<()> {
    // Link patches for ESP-IDF compatibility
    esp_idf_sys::link_patches();

    println!("Epaper init");
    let peripherals = Peripherals::take().unwrap();

    // Initialize necessary pins
    let mut delay = Ets;
    let cs = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let dc = PinDriver::output(peripherals.pins.gpio22).unwrap();
    let busy = PinDriver::input(peripherals.pins.gpio4).unwrap();
    let rst = PinDriver::output(peripherals.pins.gpio21).unwrap();
    let sclk = peripherals.pins.gpio18;
    let serial_out = peripherals.pins.gpio23;

    // Initialize SPI2
    let driver = SpiDriver::new::<SPI2>(
        peripherals.spi2,
        sclk,
        serial_out,
        Option::<gpio::Gpio2>::None,
        Dma::Disabled,
    )
        .unwrap();

    // Configure SPI2
    let config_1 = config::Config::new().baudrate(2.MHz().into());
    let mut spi = SpiDeviceDriver::new(&driver, Option::<gpio::Gpio5>::None, &config_1)?;

    // Initialize e-paper display
    let mut epd = epd4in2::Epd4in2::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();
    println!("Epaper init done");

    // Create a display buffer
    let mut buffer = vec![DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH as usize / 8 * HEIGHT as usize];
    let mut display = VarDisplay::new(WIDTH, HEIGHT, &mut buffer);

    // Define text style
    let style = MonoTextStyle::new(&FONT_10X20, Black);

    // Draw text on the display
    Text::new("Hello Rust! from Mazda", Point::new(20, 30), style).draw(&mut display)?;

    // Draw a line on the display
    let _ = Line::new(Point::new(0, 120), Point::new(0, 295))
        .into_styled(PrimitiveStyle::with_stroke(Black, 1))
        .draw(&mut display);

    // Update the e-paper display
    epd.update_frame(&mut spi, &display.buffer(), &mut delay)?;
    epd.display_frame(&mut spi, &mut delay)?;

    // Set the e-paper display to sleep
    epd.sleep(&mut spi, &mut delay)?;

    Ok(())
}
