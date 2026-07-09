// New file
use super::{ParameterID, CommandID, SEND_HEADER, SEND_TAIL};

pub struct SerialCmd<const S:usize,const R:usize>{

    pub send: [u8;S],
    pub result: [u8;R],
    pub wait_ms: u32,

}
///
//send FD FC FB FA 08 00 07 00 01 00 02 00 00 00 04 03 02 01
//result ACK FD FC FB FA_ 04 00 _07 01_ 00 00 04 03 02 01
impl SerialCmd<18,14>{


    pub fn new_set_param(param_id:ParameterID, param_value:f32) -> Self{

        let cmd_id_2b = CommandID::WriteParam.get_bytes();
        let cmd_id_ack_2b = CommandID::WriteParamAck.get_bytes();

        let param_value_4b =  match &param_id {

            ParameterID::Range => (param_value as u32).to_le_bytes(),
            ParameterID::Delay => (param_value as u32).to_le_bytes(),
            _ => encode_threshold_value_to_le_bytes(param_value),
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
            result:[
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_ms: 100,

        }

    }
}
///
//send FD FC FB FA 04 00 08 00 01 00 04 03 02 01
//result ACK FD FC FB FA 08 00 08 01 00 00  0F 00 00 00  04 03 02 01
impl SerialCmd<14,0>{


    pub fn send_read_param_value(param_id:ParameterID) -> Self{

        let cmd_id_2b = CommandID::ReadParam.get_bytes();
        let param_id_2b = param_id.get_bytes();
        //let param_value_4b = encode(param_value);

        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                param_id_2b[0],param_id_2b[1],
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result: [],
            wait_ms: 100,
        }

    }
}
///
//sent FD FC FB FA 04 00 FF 00 02 00 04 03 02 01
//result ACK FD FC FB FA 08 00 FF 01 00 00 02 00 20 00 04 03 02 01
impl SerialCmd<14,18>{
    pub fn new_begin_config( ) -> Self{

        let cmd_id_2b = CommandID::EnableConfig.get_bytes();
        let cmd_id_ack_2b = CommandID::EnableConfigAck.get_bytes();

        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                0x02, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x08, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_ms: 100,
        }
    }
}


///
//send FD FC FB FA 02 00 FE 00 04 03 02 01
//receieve ACK FD FC FB FA 04 00 FE 01 00 00 04 03 02 01
impl SerialCmd<12,14>{
    pub fn new_end_save_config( ) -> Self{

        let cmd_id_2b = CommandID::EndSaveConfig.get_bytes();
        let cmd_id_ack_2b = CommandID::EndSaveConfigAck.get_bytes();


        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x02, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_ms: 100,
        }
    }

}

///
//send FD FC FB FA 08 00 12 00 00 00 04 00 00 00 04 03 02 01
//result: 45 byte
//F4 F3 F2 F1
//23 00 //2 bytes detection result, target distance, and energy values for each distance gate
//01 // 1 byte, 00 absent, 01 present
//07 00 //2 bytes indicating the distance of the target phase from the radar in the scene
//DA A3 C9 D8 39 08 12 00 28 00 94 00 44 00 91 00 31 00 7A 00 6D 00 52 00 6D 00 35 00 65 00 41 00
///16 (total number of distance gates) * 2 bytes, size of energy value for each distance gate from 0 to 15
//F8 F7 F6 F5
impl SerialCmd<18,0>{
    pub fn new_set_report_mode() -> Self{

        let cmd_id_2b = CommandID::ReportMode.get_bytes();


        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x08, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                0x00, 0x00,0x04, 0x00,0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result: [],
            wait_ms: 100,
        }
    }

}






pub fn encode_threshold_value_to_le_bytes(value: f32) -> [u8;4] {
    if value == 0.0 {
        return [0x00,0x00,0x00,0x00];
    }
    (libm::powf(10.0, value / 10.0) as u32).to_le_bytes()
}

/*

FD FC FB FA 04 00 FF 00 02 00 04 03 02 01  -> FD FC FB FA_ 08 00_ FF 01 00 00 02 00 20 00 _ 04 03 02 01
FD FC FB FA 04 00 FF 00 02 00 04 03 02 01
FD FC FB FA 08 00 07 00 01 00 02 00 00 00 04 03 02 01 -> FD FC FB FA_ 04 00 _07 01_ 00 00 04 03 02 01 
FD FC FB FA 08 00 07 00 04 00 0A 00 00 00 04 03 02 01
FD FC FB FA 08 00 07 00 10 00 64 00 00 00 04 03 02 01
FD FC FB FA 02 00 FE 00 04 03 02 01  -> FD FC FB FA 04 00 FE 01 00 00 04 03 02 01



FD FC FB FA 04 00 08 00 01 00 04 03 02 01

*/
