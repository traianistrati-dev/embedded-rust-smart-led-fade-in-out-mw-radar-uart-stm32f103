// New file
use super::{ParameterID, CommandID, SEND_HEADER, SEND_TAIL};

pub struct SerialCmdWithACK<const S:usize,const R:usize>{

    pub send: [u8;S],
    pub result_ack: [u8;R],
    pub wait_micro_seconds: u32,

}

// pub struct SerialCmdDynamicResult<const S:usize>{

    // pub send: [u8;S],
    // pub wait_micro_seconds: u32,

// }
///
//send FD FC FB FA 08 00 07 00 01 00 02 00 00 00 04 03 02 01
//result ACK FD FC FB FA_ 04 00 _07 01_ 00 00 04 03 02 01
impl SerialCmdWithACK<18,14>{


    pub fn set_param_value(param_id:ParameterID, param_value:f32) -> Self{

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
            result_ack:[
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 50,

        }

    }
}


//sent FD FC FB FA 04 00 FF 00 02 00 04 03 02 01
//result ACK FD FC FB FA 08 00 FF 01 00 00 02 00 20 00 04 03 02 01
impl SerialCmdWithACK<14,18>{
    pub fn begin_config( ) -> Self{

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
            result_ack: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x08, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 50,
        }
    }
}


///
//send FD FC FB FA 02 00 FE 00 04 03 02 01
//receieve ACK FD FC FB FA 04 00 FE 01 00 00 04 03 02 01
impl SerialCmdWithACK<12,14>{
    pub fn end_save_config( ) -> Self{

        let cmd_id_2b = CommandID::EndSaveConfig.get_bytes();
        let cmd_id_ack_2b = CommandID::EndSaveConfigAck.get_bytes();


        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x02, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result_ack: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x04, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 50,
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
