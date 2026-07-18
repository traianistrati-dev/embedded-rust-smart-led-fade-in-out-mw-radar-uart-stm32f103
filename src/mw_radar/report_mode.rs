
use super::{Parser,ParserResult};

const CMD_HEADER: [u8; 4] = [0xF4, 0xF3, 0xF2, 0xF1];
const CMD_TAIL:   [u8; 4] = [0xF8, 0xF7, 0xF6, 0xF5];

/// Payload = 1 (status) + 2 (distance cm) + 32 (16 gates energy)
const PAYLOAD_LEN: usize = 35;

const EXPECTED_CMD_ID: u16  = super::CommandID::None.raw();
const RESERVED_LEN: usize = 0;

type ParserType<'a> = Parser<'a, PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID>;


pub struct HmmdFrame {
    pub present:     bool,
    pub distance_cm: u16,
    pub energy:      [u16; 16],
}



impl <'a>ParserResult<'a, PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID, HmmdFrame> for HmmdFrame {
    fn new_parser() -> ParserType<'a> {
        ParserType::new(&CMD_HEADER, &CMD_TAIL)
    }


    fn decode(payload:&[u8]) -> Self{
        let present = payload[0] != 0;
        let distance_cm = u16::from_le_bytes([payload[1], payload[2]]);

        let mut energy = [0u16; 16];
        for i in 0..16 {
            energy[i] = u16::from_le_bytes([payload[3 + i * 2], payload[4 + i * 2]]);
        }

        Self {
            present,
            distance_cm,
            energy,
        }

    }

}


use super::{SerialCmdWithACK, CommandID, SEND_HEADER,SEND_TAIL};

/// Data sent by sensor after set report mode
//send FD FC FB FA 08 00 12 00 00 00 04 00 00 00 04 03 02 01
//result: 45 byte
//F4 F3 F2 F1
//23 00 //2 bytes detection result, target distance, and energy values for each distance gate
//01 // 1 byte, 00 absent, 01 present
//07 00 //2 bytes indicating the distance of the target phase from the radar in the scene
//DA A3 C9 D8 39 08 12 00 28 00 94 00 44 00 91 00 31 00 7A 00 6D 00 52 00 6D 00 35 00 65 00 41 00
///16 (total number of distance gates) * 2 bytes, size of energy value for each distance gate from 0 to 15
//F8 F7 F6 F5
impl SerialCmdWithACK<18,0>{
    pub fn set_report_mode() -> Self{

        let cmd_id_2b = CommandID::ReportMode.get_bytes();


        Self {
            send: [
                SEND_HEADER[0], SEND_HEADER[1], SEND_HEADER[2], SEND_HEADER[3],
                0x08, 0x00,//data lenght
                cmd_id_2b[0], cmd_id_2b[1],
                0x00, 0x00,0x04, 0x00,0x00, 0x00,
                SEND_TAIL[0], SEND_TAIL[1], SEND_TAIL[2], SEND_TAIL[3],
            ],
            result_payload_ack:[],
            wait_micro_seconds: 50,
        }
    }

}


/*


                // ── Gate cu energia maxima (cel mai activ) ────────────────────
                    // let (peak_gate, peak_energy) = frame.energy
                    // .iter()
                    // .enumerate()
                    // .fold((0usize, 0u16), |(bi, be), (i, &e)| {
                    // if e > be { (i, e) } else { (bi, be) }
                    // });

                    // let peak_cm = (peak_gate as u16 + 1) * 70;

                    // Rand 1 (y=11): "G:3 ~210cm  E:1823"
                    // G = gate peak, ~cm = distanta din gate, E = energie peak
                    // {
                    // let mut pos = 0usize;
                    // for &c in b"G:" { lbuf[pos] = c; pos += 1; }
                    // let sg = fmt_u16(peak_gate as u16, &mut buf_a);
                    // for &c in sg.as_bytes() { lbuf[pos] = c; pos += 1; }
                    // for &c in b" ~" { lbuf[pos] = c; pos += 1; }
                    // let sp = fmt_u16(peak_cm, &mut buf_b);
                    // for &c in sp.as_bytes() { lbuf[pos] = c; pos += 1; }
                    // for &c in b"cm E:" { lbuf[pos] = c; pos += 1; }
                    // let se = fmt_u16(peak_energy, &mut buf_a);
                    // for &c in se.as_bytes() { lbuf[pos] = c; pos += 1; }
                    // let t = core::str::from_utf8(&lbuf[..pos]).unwrap_or("?");
                    // pins::utils::i2c1::wtrite_to_display(&mut display,t, 11);

                    // }

                    // Rand 2 (y=22): "E0:nnn E1:nnn E2:nnn" primele 3 gate-uri
                    // (cele mai relevante pentru detectie apropiata)
                    // {
                    // let mut pos = 0usize;
                    // for i in 0..3usize {
                    // let label = [b'E', b'0' + i as u8, b':'];
                    // for &c in &label { lbuf[pos] = c; pos += 1; }
                    // let se = fmt_u16(frame.energy[i], &mut buf_a);
                    // for &c in se.as_bytes() { lbuf[pos] = c; pos += 1; }
                    // if i < 2 { lbuf[pos] = b' '; pos += 1; }
                    // }
                    // let t = core::str::from_utf8(&lbuf[..pos]).unwrap_or("?");
                    // pins::utils::i2c1::wtrite_to_display(&mut display,t, 22);
                    // }

*/

