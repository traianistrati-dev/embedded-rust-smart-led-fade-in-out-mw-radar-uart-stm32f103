// New file
use super::{Parser, ParserResult};

const CMD_HEADER: [u8; 4] = [0xFD, 0xFC, 0xFB, 0xFA];
const CMD_TAIL:   [u8; 4] = [0x04, 0x03, 0x02, 0x01];

const PAYLOAD_LEN: usize = 4;
const EXPECTED_CMD_ID: u16  = super::CommandID::ReadParamAck.raw();
const RESERVED_LEN: usize = 2;

type ParserType = Parser<PAYLOAD_LEN,  RESERVED_LEN,EXPECTED_CMD_ID>;


pub struct ReadParam;


impl ParserResult<PAYLOAD_LEN, RESERVED_LEN,EXPECTED_CMD_ID, u32> for ReadParam {
    fn new_parser() -> ParserType {
        ParserType::new(CMD_HEADER, CMD_TAIL)
    }

    // fn payload(parser:&ParserType) -> [u8; PAYLOAD_LEN]{
    // parser.payload
    // }

    fn decode(payload:&[u8]) -> u32{
        // u32::from_le_bytes([payload[0],payload[1],payload[2],payload[3]])
        u32::from_le_bytes([payload[0],payload[1],payload[2],payload[3]])
        //u32::from_be_bytes([payload[0],payload[1],payload[2],payload[3]])
        // 0x00000001
    }
}

/*
ENTER CONFIG MOD
FD FC FB FA 04 00 FF 00 02 00 04 03 02 01

//tx send get param 01 00 value 	         rx 02 00 00 00
FD FC FB FA 04 00 08 00 01 00 04 03 02 01 -> FD FC FB FA 08 00 08 01 00 00 02 00 00 00 04 03 02 01 
*/