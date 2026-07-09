// New file
use embedded_graphics::{
    mono_font::{ascii::*, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};


use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use stm32f1xx_hal::i2c::BlockingI2c;
use stm32f1xx_hal::pac::I2C1;

pub fn get_display_128x32<PINS>(
    i2c: BlockingI2c<I2C1, PINS>,
) -> Ssd1306<
I2CInterface<BlockingI2c<I2C1, PINS>>,
DisplaySize128x32,
BufferedGraphicsMode<DisplaySize128x32>,
> {
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();

    display.init().unwrap();

    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();
    display
}


pub fn clear_display<PINS>(
    display: &mut Ssd1306<
    I2CInterface<BlockingI2c<I2C1, PINS>>,
    DisplaySize128x32,
    BufferedGraphicsMode<DisplaySize128x32>,
    >){
    display.clear(BinaryColor::Off).unwrap();
    }

pub fn wtrite_to_display<PINS>(
    display: &mut Ssd1306<
    I2CInterface<BlockingI2c<I2C1, PINS>>,
    DisplaySize128x32,
    BufferedGraphicsMode<DisplaySize128x32>,
    >,
    text: &str,
    poin_y:i32
) {
    //display.clear(BinaryColor::Off).unwrap();
    /**/
    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    Text::new(text, Point::new(0, poin_y+6), style)
    .draw(display)
    .unwrap();
    /**/

    //display.flush().unwrap();
    
}

pub fn wtrite_to_display_font10x20<PINS>(
    display: &mut Ssd1306<
    I2CInterface<BlockingI2c<I2C1, PINS>>,
    DisplaySize128x32,
    BufferedGraphicsMode<DisplaySize128x32>,
    >,
    text: &str,
    poin_y:i32
) {
    //display.clear(BinaryColor::Off).unwrap();
    /**/
    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    Text::new(text, Point::new(0, poin_y+10), style)
    .draw(display)
    .unwrap();
    /**/

    //display.flush().unwrap();
    
}



// pub fn write_128x32_text<PINS>( display: &mut Ssd1306<
    // I2CInterface<BlockingI2c<I2C1, PINS>>,
    // DisplaySize128x32,
    // BufferedGraphicsMode<DisplaySize128x32>,
    // >, text: &str) {


    // wtrite_to_display(display, text, 16);

// }
// pub fn write_128x32_text_2lines<PINS>(  display: &mut Ssd1306<
    // I2CInterface<BlockingI2c<I2C1, PINS>>,
    // DisplaySize128x32,
    // BufferedGraphicsMode<DisplaySize128x32>,
    // >, text_line1: &str, text_line2: &str) {


    // wtrite_to_display(display, text_line1, 10);
    // wtrite_to_display(display, text_line2, 24);

// }

// pub fn write_128x32_hello_world<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C1, PINS>,
// ) {
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(
        // interface,
        // DisplaySize128x32,
        // //DisplaySize128x64,
        // DisplayRotation::Rotate0,
    // )
    // .into_buffered_graphics_mode();

    // display.init().unwrap();

    // display.clear(BinaryColor::Off).unwrap();
    // /**/
    // let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    // Text::new("Hello World!", Point::new(0, 16), style)
    // .draw(&mut display)
    // .unwrap();
    // /**/

    // display.flush().unwrap();
// }

// pub fn write_128x64_hello_world<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C1, PINS>,
// ) {
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(
        // interface,
        // //DisplaySize128x32,
        // DisplaySize128x64,
        // DisplayRotation::Rotate0,
    // )
    // .into_buffered_graphics_mode();

    // display.init().unwrap();

    // display.clear(BinaryColor::Off).unwrap();
    // /**/
    // let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    // Text::new("Hello World!", Point::new(0, 16), style)
    // .draw(&mut display)
    // .unwrap();
    // /**/
    // /*
    // for _ in 0..100000 {
        // cortex_m::asm::nop();
    // }
    // */
    // /*
    // for y in 0..64 {
        // for x in 0..128 {
            // if (x / 8) % 2 == 0 {
                // Pixel(
                    // Point::new(x, y),
                    // BinaryColor::On,
                // )
                // .draw(&mut display)
                // .unwrap();
            // }
        // }
    // }
    // */

    // display.flush().unwrap();
// }

// pub fn write_128x64_rectangle<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C1, PINS>,
// ) {
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(
        // interface,
        // //DisplaySize128x32,
        // DisplaySize128x64,
        // DisplayRotation::Rotate0,
    // )
    // .into_buffered_graphics_mode();

    // display.init().unwrap();

    // display.clear(BinaryColor::Off).unwrap();

    // embedded_graphics::primitives::Rectangle::new(Point::new(0, 0), Size::new(20, 20))
    // .into_styled(embedded_graphics::primitives::PrimitiveStyle::with_fill(
            // BinaryColor::On,
    // ))
    // .draw(&mut display)
    // .unwrap();

    // display.flush().unwrap();
// }

// pub fn write_128x32_rectangle<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C1, PINS>,
// ) {
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(
        // interface,
        // DisplaySize128x32,
        // //DisplaySize128x64,
        // DisplayRotation::Rotate0,
    // )
    // .into_buffered_graphics_mode();

    // display.init().unwrap();

    // display.clear(BinaryColor::Off).unwrap();

    // embedded_graphics::primitives::Rectangle::new(Point::new(0, 0), Size::new(20, 20))
    // .into_styled(embedded_graphics::primitives::PrimitiveStyle::with_fill(
            // BinaryColor::On,
    // ))
    // .draw(&mut display)
    // .unwrap();

    // display.flush().unwrap();
// }
