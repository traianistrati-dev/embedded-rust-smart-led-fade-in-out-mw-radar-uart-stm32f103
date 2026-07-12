// use stm32f1xx_hal::digital::InputPin;

// pub enum EncoderEvent {
    // None,
    // Left,
    // Right,
    // Pressed,
// }

// pub struct RotaryEncoder<CLK, DT, SW>
// where
    // CLK: InputPin,
    // DT: InputPin,
    // SW: InputPin,
// {
    // clk: CLK,
    // dt: DT,
    // sw: SW,
    // last_clk: bool,
    // last_sw: bool,
    // pub value: i32,
// }

// impl<CLK, DT, SW> RotaryEncoder<CLK, DT, SW>
// where
    // CLK: InputPin,
    // DT: InputPin,
    // SW: InputPin,
// {
    // pub fn new(clk: CLK, dt: DT, sw: SW) -> Self {
        // let last_clk = clk.is_high().ok().unwrap_or(false);
        // let last_sw = sw.is_high().ok().unwrap_or(false);

        // Self {
            // clk,
            // dt,
            // sw,
            // last_clk,
            // last_sw,
            // value: 0,
        // }
    // }

    // pub fn update(&mut self) -> EncoderEvent {
        // let clk = self.clk.is_high().ok().unwrap_or(false);
        // let dt = self.dt.is_high().ok().unwrap_or(false);
        // let sw = self.sw.is_high().ok().unwrap_or(false);

        // if self.last_sw && !sw {
            // self.last_sw = sw;
            // self.last_clk = clk;
            // return EncoderEvent::Pressed;
        // }

        // if self.last_clk && !clk {
            // self.last_clk = clk;
            // self.last_sw = sw;

            // if dt != clk {
                // self.value += 1;
                // return EncoderEvent::Right;
            // } else {
                // self.value -= 1;
                // return EncoderEvent::Left;
            // }
        // }

        // self.last_clk = clk;
        // self.last_sw = sw;
        // EncoderEvent::None
    // }
// }