// // New file
// // New file
// use embedded_graphics::{
    // mono_font::{ascii::*, MonoTextStyle},
    // pixelcolor::BinaryColor,
    // prelude::*,
    // text::Text,
// };

// use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

// //use stm32f1xx_hal::i2c::BlockingI2c;

// pub fn write_128x32_hello_world<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C2, PINS>,
// ) {
  
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(interface,
     // DisplaySize128x32,
     // //DisplaySize128x64,
      // DisplayRotation::Rotate0)
        // .into_buffered_graphics_mode();

    // display.init().unwrap();

    // display.clear(BinaryColor::Off).unwrap();
// /**/
    // let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

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
// for y in 0..32 {
    // for x in 0..128 {
        // if (x / 8) % 4 == 0 {
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

// pub fn write_128x64_hello_world<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C2, PINS>,
// ) {
  
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(interface,
     // //DisplaySize128x32,
     // DisplaySize128x64,
      // DisplayRotation::Rotate0)
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
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C2, PINS>,
// ) {
  
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(interface,
     // //DisplaySize128x32,
     // DisplaySize128x64,
      // DisplayRotation::Rotate0)
        // .into_buffered_graphics_mode();

// display.init().unwrap();

// display.clear(BinaryColor::Off).unwrap();

// embedded_graphics::primitives::Rectangle::new(
    // Point::new(0, 0),
    // Size::new(20, 20),
// )
// .into_styled(
    // embedded_graphics::primitives::PrimitiveStyle::with_fill(BinaryColor::On),
// )
// .draw(&mut display)
// .unwrap();

// display.flush().unwrap();




// }


// pub fn write_128x32_rectangle<PINS>(
    // i2c: stm32f1xx_hal::i2c::BlockingI2c<stm32f1xx_hal::pac::I2C2, PINS>,
// ) {
  
    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(interface,
     // DisplaySize128x32,
     // //DisplaySize128x64,
      // DisplayRotation::Rotate0)
        // .into_buffered_graphics_mode();

// display.init().unwrap();

// display.clear(BinaryColor::Off).unwrap();

// embedded_graphics::primitives::Rectangle::new(
    // Point::new(0, 0),
    // Size::new(20, 20),
// )
// .into_styled(
    // embedded_graphics::primitives::PrimitiveStyle::with_fill(BinaryColor::On),
// )
// .draw(&mut display)
// .unwrap();

// display.flush().unwrap();




// }
