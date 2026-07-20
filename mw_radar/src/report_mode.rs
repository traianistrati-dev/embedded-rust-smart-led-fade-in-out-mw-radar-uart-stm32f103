
use super::{Parser,ParserResult};

const CMD_HEADER: [u8; 10] = [0x4F, 0x4E, //ON
    0x0D, 0x0A,//\n
    0x52, 0x61, 0x6E, 0x67,0x65,//Range
    0x20 // SP
];
const CMD_TAIL: [u8; 2] = [0x0D, 0x0A]; //\n


/// maxuimum 4 ASCII byte
const PAYLOAD_LEN: usize = 0;

const EXPECTED_CMD_ID: u16 = super::CommandID::None.raw();
const RESERVED_LEN: usize = 0;

type ParserType<'a> = Parser<'a, PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID>;


pub struct HmmdAsciiFrame{
    // pub distance_cm: [char;4]
    pub distance_cm: [u8;4]
}



impl <'a>ParserResult<'a, PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID, HmmdAsciiFrame> for HmmdAsciiFrame {
    fn new_parser() -> ParserType<'a> {
        ParserType::new(&CMD_HEADER, &CMD_TAIL)
    }


    fn decode(payload:&[u8]) -> Self{
        // let mut distance_cm = [' ';4];
        let mut distance_cm = [0u8;4];
        for i in 0..4{
            // distance_cm[i] = payload[i] as char;
            distance_cm[i] = payload[i] ;
        }


        Self {
            distance_cm
        }

    }

}


use super::{SerialCmd, CommandID, SEND_HEADER,SEND_TAIL};

/// Set report normal mode
//send FD FC FB FA 08 00 12 00 00 00 64 00 00 00 04 03 02 01
//result: ASCII bytes
impl SerialCmd<18,0>{
    pub fn set_report_ascii_mode() -> Self{

        let cmd_id_2b: [u8; 2] = CommandID::ReportMode.get_bytes();


        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x08, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                0x00, 0x00,0x64, 0x00,0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result_payload_ack:[],
            wait_micro_seconds: 50,
        }
    }

}


