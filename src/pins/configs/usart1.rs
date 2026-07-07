// <<< GENERATED>>>
// Peripheral config (from the Virtual Module) — auto-updated; edit in the module.
const BAUDRATE: u32 = 115200;
const DATA_BITS: u8 = 8; // 8, 9
const PARITY: char = 'N'; // 'N' None, 'O' Odd, 'E' Even
const STOP_BITS: u8 = 1; // 1, 2
// <<< GENERATED END >>>

use stm32f1xx_hal::{
    pac,
    prelude::*,
    afio,
    rcc::Clocks,
    serial::{self, Config, Serial,StopBits},
};


fn get_config() -> stm32f1xx_hal::serial::Config{


    let mut config = Config::default().baudrate(BAUDRATE.bps());

    if DATA_BITS == 8{
        config = config.wordlength_8bits();
    }
    else if DATA_BITS == 9{
        config = config.wordlength_9bits();
    }

    if PARITY=='N'{
        config = config.parity_none();
    }
    else if PARITY=='O'{
        config = config.parity_odd();
    }
    else if PARITY=='E'{
        config = config.parity_even();
    }


    if STOP_BITS == 1{
        config = config.stopbits(StopBits::STOP1);
    }
    else if STOP_BITS == 2{
        config = config.stopbits(StopBits::STOP2);
    }

    config
}



pub fn init(
    usart: pac::USART1,
    pins: impl serial::Pins<pac::USART1>,
    afio: &mut afio::Parts,
    clocks: &Clocks,
) -> (serial::Tx<pac::USART1>, serial::Rx<pac::USART1>) {
    Serial::new(
        usart,
        pins,
        &mut afio.mapr,
        get_config(),
        clocks,
    )
    .split()
}