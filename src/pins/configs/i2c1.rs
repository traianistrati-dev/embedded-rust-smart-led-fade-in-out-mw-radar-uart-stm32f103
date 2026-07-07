// <<< GENERATED>>>
// Peripheral config (from the Virtual Module) — auto-updated; edit in the module.
const CLOCK_KHZ: u32 = 400; // <=100 Standard, >100 Fast
// <<< GENERATED END >>>

// Everything below is editable — your changes are preserved on regeneration.
use stm32f1xx_hal::{
    pac,
    prelude::*,
    afio,
    rcc::Clocks,
    i2c::{self, BlockingI2c, Mode as I2cMode},
};

fn get_mode() -> I2cMode {
    if CLOCK_KHZ <= 100 {
        I2cMode::Standard { frequency: CLOCK_KHZ.kHz() }
    } else {
        I2cMode::Fast { frequency: CLOCK_KHZ.kHz(), duty_cycle: i2c::DutyCycle::Ratio2to1 }
    }
}

pub fn init<PINS>(
    i2c: pac::I2C1,
    pins: PINS,
    afio: &mut afio::Parts,
    clocks: &Clocks,
) -> BlockingI2c<pac::I2C1, PINS>
where
    PINS: i2c::Pins<pac::I2C1>,
{
    BlockingI2c::i2c1(i2c, pins, &mut afio.mapr, get_mode(), *clocks, 1000, 10, 1000, 1000)
}
