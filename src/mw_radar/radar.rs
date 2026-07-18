use stm32f1xx_hal::pac;

type UsartTxType = stm32f1xx_hal::serial::Tx<pac::USART1>;
type UsartRxType = stm32f1xx_hal::serial::Rx<pac::USART1>;

// <>


use super::{ParameterID, SerialCmdWithACK, ParserResult};

pub struct MicrowaveRadar<DELAY:DelayMs>{

    delay: DELAY,
    tx: UsartTxType,
    rx: UsartRxType,

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

impl <DELAY:DelayMs> MicrowaveRadar<DELAY>{


    pub fn new(tx: UsartTxType,rx: UsartRxType, delay_fn: DELAY) -> Self {
        Self { delay:delay_fn, tx,rx}
    }

    pub fn read_byte(&mut self,mut read_fn:impl FnMut(u8)){
        if let Ok(b) = self.rx.read() {
            read_fn(b);
        }
    }

    pub fn delay_micro_seconds(&self, ms:u32) {

        self.delay.delay_ms(ms);
    }

    pub fn send_config_example1(&mut self, max_range:f32, delay_sec:f32, trigger_treschold_00:f32){


        if self.begin_config(){

            if self.begin_config(){

                if self.send_cmd(SerialCmdWithACK::set_param_value(ParameterID::Range, max_range)){

                    if self.send_cmd(SerialCmdWithACK::set_param_value(ParameterID::Delay, delay_sec)){
                        if self.send_cmd(SerialCmdWithACK::set_param_value(ParameterID::TriggerThreshold00, trigger_treschold_00)){

                            if self.end_save_config(){
                                self.send_cmd(SerialCmdWithACK::set_report_mode());
                            }
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
            SerialCmdWithACK::read_param_value(param_id)
            ,parser
            , super::parameter::ReadParam::decode
        )

    }



    pub fn send_cmd_and_get_result<const S:usize,const PAYLOAD_LEN: usize, const RESERVED_LEN: usize, const EXPECTED_CMD_ID: u16, RESULT>(
        &mut self,
        data:SerialCmdWithACK<S,0>,
        parser: &mut super::Parser<PAYLOAD_LEN,RESERVED_LEN,EXPECTED_CMD_ID>,
        decoder: fn(&[u8]) -> RESULT,
        //parser2:impl super::ParserResult<PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID, RESULT>

    ) -> Option<RESULT>
    {
        {//send data to tx
            let tx =  &mut self.tx;

            for &b in &data.send {
                nb::block!(tx.write(b)).ok();
            }
            tx.flush().unwrap_or_default();
        }

        self.delay_micro_seconds(data.wait_micro_seconds);

        {//read data from rx
            parser.clear();

            let mut idle_loops = 0u32;

            let rx =  &mut self.rx;
            loop {
                match rx.read() {
                    Ok(b) => {
                        idle_loops = 0;

                        if parser.feed(b) {
                            return Some(decoder(&parser.payload));
                        }
                    }

                    Err(nb::Error::WouldBlock) => {
                        idle_loops += 1;
                        if idle_loops > 50_000 {
                            break;
                        }
                    }

                    Err(_) => {
                        break;
                    }
                }
            }
        }

        None

    }


    pub fn send_cmd<'a, const S:usize, const R:usize>(
        &mut self,
        data:SerialCmdWithACK<S,R>,

    ) -> bool{
        {//send data to tx
            let tx =  &mut self.tx;
            for &b in &data.send {
                nb::block!(tx.write(b)).ok();
            }
            tx.flush().unwrap_or_default();
        }

        self.delay_micro_seconds(data.wait_micro_seconds);

        {//read data from rx
            if !data.result_payload_ack.is_empty() {


                {//read data from rx
                    let mut parser = super::Parser::<'a, R, 0, { super::CommandID::None.raw() }>::new(&super::SEND_HEADER, &super::SEND_TAIL);

                    parser.clear();

                    let mut idle_loops = 0u32;

                    let rx =  &mut self.rx;
                    loop {
                        match rx.read() {
                            Ok(b) => {
                                idle_loops = 0;

                                if parser.feed(b) {

                                    for i in 0..R{
                                        if data.result_payload_ack[i] != parser.payload[i] {
                                            return false;
                                        }
                                    }
                                    return true;
                                }
                            }

                            Err(nb::Error::WouldBlock) => {
                                idle_loops += 1;
                                if idle_loops > 50_000 {
                                    break;
                                }
                            }

                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            }
            return false;
        }
        true
    }



    pub fn begin_config(&mut self) -> bool{
        self.send_cmd(SerialCmdWithACK::begin_config())
    }

    pub fn end_save_config(&mut self) -> bool{
        self.send_cmd(SerialCmdWithACK::end_save_config())
    }


}


