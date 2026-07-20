
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

    pub fn read_byte(&mut self,mut read_fn:impl FnMut(u8)){
        if let Some(b) = self.rx_read.read_byte() {
            read_fn(b);
        }
    }





    pub fn delay_micro_seconds(&self, ms:u32) {

        self.delay.delay_ms(ms);
    }

    pub fn send_config_example1(&mut self, max_range:f32, delay_sec:f32, trigger_treschold_00:f32){


        if self.begin_config() && self.begin_config(){

            if self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::Range, max_range)){

                if self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::Delay, delay_sec)){
                    if self.send_cmd_and_check_ack_result(SerialCmd::set_param_value(ParameterID::TriggerThreshold00, trigger_treschold_00)){

                        if self.end_save_config(){

                           self.send_cmd_and_check_ack_result(SerialCmd::set_report_mode());
                           //self.send_cmd_and_check_ack_result(SerialCmd::set_report_ascii_mode());
                        }
                    }

                }

            }

        }



        // self.begin_config();
        // self.begin_config();

        // self.send_cmd(SerialCmdWithACK::set_param_value(ParameterID::Range, max_range));
        // self.send_cmd(SerialCmdWithACK::set_param_value(ParameterID::Delay, delay_sec));
        // self.send_cmd(SerialCmdWithACK::set_param_value(ParameterID::TriggerThreshold00, trigger_treschold_00));

        // self.send_cmd(SerialCmdWithACK::set_report_mode());

        // self.end_save_config();

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


    pub fn send_cmd_and_check_ack_result<'a, const S:usize, const R:usize>(
        &mut self,
        data:SerialCmd<S,R>,

    ) -> bool{
        self.tx_write.write_bytes(&data.send);

        self.delay_micro_seconds(data.wait_micro_seconds);


        if !data.result_payload_ack.is_empty() {

            let mut parser = super::Parser::<'a, R, 0, { super::CommandID::None.raw() }>::new(&super::SEND_HEADER, &super::SEND_TAIL);

            parser.clear();

            let mut idle_loops = 0u32;

            loop {

                if let Some(b) = self.rx_read.read_byte() {
                    if parser.feed(b) {
                        // return Some(decoder(&parser.payload));

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


