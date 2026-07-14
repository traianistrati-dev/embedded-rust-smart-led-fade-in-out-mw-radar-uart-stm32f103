// New file
use super::{Parser, ParserResult};

const CMD_HEADER: [u8; 4] = SEND_HEADER;//[0xFD, 0xFC, 0xFB, 0xFA];
const CMD_TAIL:   [u8; 4] = SEND_TAIL;//[0x04, 0x03, 0x02, 0x01];

const PAYLOAD_LEN: usize = 4;
const EXPECTED_CMD_ID: u16  = super::CommandID::ReadParamAck.raw();
const RESERVED_LEN: usize = 2;

type ParserType = Parser<PAYLOAD_LEN, RESERVED_LEN,EXPECTED_CMD_ID>;


pub struct ReadParam;


impl ParserResult<PAYLOAD_LEN, RESERVED_LEN,EXPECTED_CMD_ID, u32> for ReadParam {
    fn new_parser() -> ParserType {
        ParserType::new(CMD_HEADER, CMD_TAIL)
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

///
//send FD FC FB FA 04 00 08 00 01 00 04 03 02 01
//result ACK FD FC FB FA 08 00 08 01 00 00  0F 00 00 00  04 03 02 01
impl SerialCmdWithACK<14,0>{


    pub fn send_read_param_value(param_id:ParameterID) -> Self{

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
            result_ack:[]
        }

    }
}