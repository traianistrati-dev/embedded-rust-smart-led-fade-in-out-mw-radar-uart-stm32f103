
use super::{ParameterID, SerialCmd, ParserResult};

pub struct MicrowaveRadar<DELAY:DelayMs,TX:UsartTx,RX:UsartRx>{

    delay: DELAY,
    tx_write:TX,
    rx_read:RX,

}

pub trait UsartTx {
    fn write_bytes(&mut self, data: &[u8]);
}

impl<F> UsartTx for F where F: FnMut(&[u8]),
{
    fn write_bytes(&mut self, data: &[u8]){
        self(data);
    }
}

pub trait UsartRx {
    fn read_byte(&mut self) -> Option<u8>;
}

impl<F> UsartRx for F where F: FnMut() -> Option<u8>,
{
    fn read_byte(&mut self)-> Option<u8>{
        self()
    }
}


pub trait DelayMs {
    fn delay_ms(&self, ms: u32);
}

impl<F> DelayMs for F where F: Fn(u32),
{
    fn delay_ms(&self, ms: u32) {
        self(ms);
    }
}

impl <DELAY:DelayMs, TX:UsartTx,RX:UsartRx> MicrowaveRadar<DELAY,TX,RX>{


    pub fn new(delay_fn: DELAY, tx_write:TX,rx_read:RX) -> Self {
        Self { delay:delay_fn,tx_write,rx_read}
    }


    ///!!! All parameters must be set before end_save_config() prior to save configuration on sensor flash memory !!!
    pub fn set_range_delay_with_default_threshold(&mut self, max_range:f32, delay_sec:f32){


        if self.begin_config() && self.begin_config()

        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::Range, max_range))

        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::Delay, delay_sec))

        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold00, 48.93))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold00, 47.38))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold01, 45.57))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold01, 44.03))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold02, 43.20))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold02, 41.66))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold03, 36.18))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold03, 34.63))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold04, 34.45))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold04, 32.90))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold05, 32.04))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold05, 30.49))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold06, 30.22))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold06, 28.67))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold07, 27.90))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold07, 26.35))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold08, 25.86))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold08, 24.31))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold09, 23.45))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold09, 21.90))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold10, 21.90))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold10, 20.35))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold11, 21.37))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold11, 19.82))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold12, 19.98))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold12, 18.44))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold13, 20.05))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold13, 18.50))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold14, 18.98))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold14, 17.43))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold15, 18.75))
        && self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::HoldThreshold15, 17.20))


        && self.end_save_config(){


        }

        self.send_cmd_and_check_ack_result(SerialCmd::set_report_mode());

    }


    pub fn read_byte(&mut self,mut read_fn:impl FnMut(u8)){
        if let Some(b) = self.rx_read.read_byte() {
            read_fn(b);
        }
    }


    pub fn delay_micro_seconds(&self, ms:u32) {

        self.delay.delay_ms(ms);
    }


    pub fn get_param_value<const PAYLOAD_LEN: usize, const RESERVED_LEN: usize, const EXPECTED_CMD_ID: u16>(
        &mut self
        ,param_id:ParameterID
        ,parser:&mut super::Parser<PAYLOAD_LEN,RESERVED_LEN,EXPECTED_CMD_ID>
    ) -> Option<u32>{


        self.send_cmd_and_get_result(
            SerialCmd::read_param_value(param_id)
            ,parser
            , super::parameter::ReadParam::decode
        )

    }



    pub fn send_cmd_and_get_result<const S:usize,const PAYLOAD_LEN: usize, const RESERVED_LEN: usize, const EXPECTED_CMD_ID: u16, RESULT>(
        &mut self,
        data:SerialCmd<S,0>,
        parser: &mut super::Parser<PAYLOAD_LEN,RESERVED_LEN,EXPECTED_CMD_ID>,
        decoder: fn(&[u8]) -> RESULT,
    ) -> Option<RESULT>
    {
        self.tx_write.write_bytes(&data.send);

        self.delay_micro_seconds(data.wait_micro_seconds);

        parser.clear();

        let mut idle_loops = 0u32;

        loop {

            if let Some(b) = self.rx_read.read_byte() {
                if parser.feed(b) {
                    return Some(decoder(&parser.payload));
                }
            }else{

                idle_loops += 1;
                if idle_loops > 50_000 {
                    break;
                }
            }

        }

        None

    }


    pub fn send_cmd_and_check_ack_result<'a, const S:usize, const R:usize>(&mut self, data:SerialCmd<S,R>) -> bool{
        self.tx_write.write_bytes(&data.send);

        self.delay_micro_seconds(data.wait_micro_seconds);


        if !data.result_payload_ack.is_empty() {

            let mut parser = super::Parser::<'a, R, 0, { super::CommandID::None.as_u16() }>::new(&super::SEND_HEADER, &super::SEND_TAIL);

            parser.clear();

            let mut idle_loops = 0u32;

            loop {

                if let Some(b) = self.rx_read.read_byte() {
                    if parser.feed(b) {

                        for i in 0..R{
                            if data.result_payload_ack[i] != parser.payload[i] {
                                return false;
                            }
                        }
                        return true;
                    }
                }else{

                    idle_loops += 1;
                    if idle_loops > 50_000 {
                        break;
                    }
                }
            }

        }
        false

    }



    pub fn begin_config(&mut self) -> bool{
        self.send_cmd_and_check_ack_result(SerialCmd::begin_config())
    }

    pub fn end_save_config(&mut self) -> bool{
        self.send_cmd_and_check_ack_result(SerialCmd::end_save_config())
    }


}


