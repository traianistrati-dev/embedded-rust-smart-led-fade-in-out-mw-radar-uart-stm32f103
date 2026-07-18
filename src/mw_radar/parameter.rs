
use super::{Parser, ParserResult};

const CMD_HEADER: [u8; 4] = SEND_HEADER;//[0xFD, 0xFC, 0xFB, 0xFA];
const CMD_TAIL:   [u8; 4] = SEND_TAIL;//[0x04, 0x03, 0x02, 0x01];

const PAYLOAD_LEN: usize = 4;
const EXPECTED_CMD_ID: u16  = super::CommandID::ReadParamAck.raw();
const RESERVED_LEN: usize = 2;

type ParserType<'a> = Parser<'a, PAYLOAD_LEN, RESERVED_LEN,EXPECTED_CMD_ID>;


pub struct ReadParam;


impl <'a>ParserResult<'a, PAYLOAD_LEN, RESERVED_LEN,EXPECTED_CMD_ID, u32> for ReadParam {
    fn new_parser() -> ParserType<'a> {
        ParserType::new(&CMD_HEADER, &CMD_TAIL)
    }


    fn decode(payload:&[u8]) -> u32{
        u32::from_le_bytes([payload[0],payload[1],payload[2],payload[3]])
    }
}

/*
ENTER CONFIG MOD
FD FC FB FA 04 00 FF 00 02 00 04 03 02 01

//tx send get param 01 00 value 	         rx 02 00 00 00
FD FC FB FA 04 00 08 00 01 00 04 03 02 01 -> FD FC FB FA 08 00 08 01 00 00 02 00 00 00 04 03 02 01 
*/

use super::{SerialCmdWithACK, ParameterID, CommandID, SEND_HEADER,SEND_TAIL};

///Reading parameter value requires this parser instance
/*
   //Code example 

	let delay_micro_seconds_fn = |ms:u32|{
        cortex_m::asm::delay(ms.saturating_mul(&clocks.sysclk().to_Hz() / 1_000_000)); //some example
    };


   let mut radar = mw_radar::MicrowaveRadar::new(_tx1_mw_radar, _rx1_mw_radar, delay_micro_seconds_fn);

   let mut parser_params = mw_radar::parameter::ReadParam::new_parser();

   let radar_range_gate_val: Option<u32> = radar.get_param_value( mw_radar::data::ParameterID::Range ,&mut parser_params);
   let radar_delay_gate_val:Option<u32> = radar.get_param_value(mw_radar::data::ParameterID::Delay,&mut parser_params);
   let radar_tt_00_val:Option<u32> = radar.get_param_value(mw_radar::data::ParameterID::TriggerThreshold00,&mut parser_params);
   let radar_ht_00_val:Option<u32> = radar.get_param_value(mw_radar::data::ParameterID::HoldThreshold00,&mut parser_params);

*/
//send FD FC FB FA 04 00 08 00 01 00 04 03 02 01
//result ACK FD FC FB FA 08 00 08 01 00 00  0F 00 00 00  04 03 02 01
impl SerialCmdWithACK<14,0>{


    pub fn read_param_value(param_id:ParameterID) -> Self{

        let cmd_id_2b = CommandID::ReadParam.get_bytes();
        let param_id_2b = param_id.get_bytes();

        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                param_id_2b[0],param_id_2b[1],
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 50,
            result_payload_ack:[]
        }

    }
}

///
//send FD FC FB FA 08 00 07 00 01 00 02 00 00 00 04 03 02 01
//result ACK FD FC FB FA_ 04 00 _07 01_ 00 00 04 03 02 01
impl SerialCmdWithACK<18,4>{


    pub fn set_param_value(param_id:ParameterID, param_value:f32) -> Self{

        let cmd_id_2b = CommandID::WriteParam.get_bytes();
        let cmd_id_ack_2b = CommandID::WriteParamAck.get_bytes();

        let param_value_4b =  match &param_id {

            ParameterID::Range => (param_value as u32).to_le_bytes(),
            ParameterID::Delay => (param_value as u32).to_le_bytes(),
            _ => super::encode_threshold_value_to_le_bytes(param_value),
        };

        let param_id_2b = param_id.get_bytes();

        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x08, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                param_id_2b[0],param_id_2b[1],
                param_value_4b[0],param_value_4b[1],param_value_4b[2],param_value_4b[3],
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result_payload_ack:[
              //  SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                //0x04, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
               // SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 50,

        }
    }
}