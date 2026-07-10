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

//////////////////// fromaters //////////////





pub fn format_text_with_u32_2inline<'a>(
    prefix: &str,
    number: f32,
    prefix_2: &str,
    number_2: f32,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 24];
    let num_str = fmt_f32( number,2, &mut num_buf);

    let mut num_buf_2 = [0u8; 24];
    let num_str_2 = fmt_f32(number_2,2,&mut num_buf_2);
    concat_4(prefix, num_str, prefix_2,num_str_2, out)
}

pub fn concat_4<'a>(
    a: &str,
    b: &str,
    c: &str,
    d: &str,
    out: &'a mut [u8],
) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let c_bytes = c.as_bytes();
    let d_bytes = d.as_bytes();

    let total_len = a_bytes.len() + b_bytes.len() + c_bytes.len()+ d_bytes.len();

    if total_len > out.len() {
        return "-";
    }

    let mut pos = 0;

    out[pos..pos + a_bytes.len()].copy_from_slice(a_bytes);
    pos += a_bytes.len();

    out[pos..pos + b_bytes.len()].copy_from_slice(b_bytes);
    pos += b_bytes.len();

    out[pos..pos + c_bytes.len()].copy_from_slice(c_bytes);
    pos += c_bytes.len();

    out[pos..pos + d_bytes.len()].copy_from_slice(d_bytes);
    pos += d_bytes.len();

    core::str::from_utf8(&out[..pos]).unwrap_or("-")
}





pub fn format_text_with_u32<'a>(
    prefix: &str,
    number: u32,
    suffix: &str,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 10];
    let num_str = fmt_u32(number, &mut num_buf);
    concat_3(prefix, num_str, suffix, out)
}





pub fn concat_3<'a>(
    a: &str,
    b: &str,
    c: &str,
    out: &'a mut [u8],
) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let c_bytes = c.as_bytes();

    let total_len = a_bytes.len() + b_bytes.len() + c_bytes.len();

    if total_len > out.len() {
        return "-";
    }

    let mut pos = 0;

    out[pos..pos + a_bytes.len()].copy_from_slice(a_bytes);
    pos += a_bytes.len();

    out[pos..pos + b_bytes.len()].copy_from_slice(b_bytes);
    pos += b_bytes.len();

    out[pos..pos + c_bytes.len()].copy_from_slice(c_bytes);
    pos += c_bytes.len();

    core::str::from_utf8(&out[..pos]).unwrap_or("-")
}




pub fn fmt_u32(mut n: u32, buf: &mut [u8; 10]) -> &str {
    if n == 0 {
        buf[0] = b'0';
        return core::str::from_utf8(&buf[..1]).unwrap();
    }

    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    core::str::from_utf8(&buf[i..]).unwrap()
}

pub fn fmt_u16(n: u16, buf: &mut [u8; 10]) -> &str {
    fmt_u32(n as u32, buf)
}




pub fn fmt_f32(value: f32, decimals: u32, buf: &mut [u8; 24]) -> &str {
    let mut pos = 0usize;
    let mut v = value;

    if v.is_sign_negative() {
        buf[pos] = b'-';
        pos += 1;
        v = -v;
    }

    let scale = match decimals {
        0 => 1u32,
        1 => 10u32,
        2 => 100u32,
        3 => 1000u32,
        4 => 10000u32,
        _ => 100u32, // fallback
    };

    let scaled = (v * scale as f32 + 0.5) as u32;
    let int_part = scaled / scale;
    let frac_part = scaled % scale;

    // partea întreagă
    let mut int_buf = [0u8; 10];
    let int_str = fmt_u32(int_part, &mut int_buf);
    let int_bytes = int_str.as_bytes();

    buf[pos..pos + int_bytes.len()].copy_from_slice(int_bytes);
    pos += int_bytes.len();

    if decimals > 0 {
        buf[pos] = b'.';
        pos += 1;

        // scriem fracția cu zero-padding
        let mut div = scale / 10;
        while div > 0 {
            buf[pos] = b'0' + ((frac_part / div) % 10) as u8;
            pos += 1;
            div /= 10;
        }
    }

    core::str::from_utf8(&buf[..pos]).unwrap()
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
