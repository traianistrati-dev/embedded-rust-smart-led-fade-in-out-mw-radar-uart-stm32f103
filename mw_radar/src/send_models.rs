use super::{CommandID, SEND_HEADER, SEND_TAIL};

pub struct SerialCmd<const S:usize,const R:usize>{

    pub send: [u8;S],
    pub result_payload_ack: [u8;R],
    pub wait_micro_seconds: u32,

}

///Used for initiating configuration setup, before setting parametrs values 
//sent FD FC FB FA 04 00 FF 00 02 00 04 03 02 01
//result ACK FD FC FB FA 08 00 FF 01 00 00 02 00 20 00 04 03 02 01
impl SerialCmd<14,8>{
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
            result_payload_ack: [
               // SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                //0x08, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
                0x02, 0x00, 0x20, 0x00,
               // SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 50,
        }
    }
}


///Used to save configuration data setup to sensor, at the end of finishing setting parametrs values 
//send FD FC FB FA 02 00 FE 00 04 03 02 01
//receieve ACK FD FC FB FA 04 00 FE 01 00 00 04 03 02 01
impl SerialCmd<12,4>{
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
            result_payload_ack: [
               // SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
               // 0x04, 0x00,//data lenght
                cmd_id_ack_2b[0], cmd_id_ack_2b[1],
                0x00, 0x00,
               // SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            wait_micro_seconds: 500,
        }
    }

}




pub fn encode_threshold_value_to_le_bytes(value: f32) -> [u8;4] {
    if value == 0.0 {
        return [0x00,0x00,0x00,0x00];
    }
    (libm::powf(10.0, value / 10.0) as u32).to_le_bytes()
}

