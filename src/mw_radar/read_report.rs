
use super::{Parser,ParserResult};

const CMD_HEADER: [u8; 4] = [0xF4, 0xF3, 0xF2, 0xF1];
const CMD_TAIL:   [u8; 4] = [0xF8, 0xF7, 0xF6, 0xF5];

/// Payload = 1 (status) + 2 (distance cm) + 32 (16 gates energy)
const PAYLOAD_LEN: usize = 35;

const EXPECTED_CMD_ID: u16  = super::CommandID::None.raw();
const RESERVED_LEN: usize = 0;

type ParserType = Parser<PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID>;


pub struct HmmdFrame {
    pub present:     bool,
    pub distance_cm: u16,
    pub energy:      [u16; 16],
}



impl ParserResult<PAYLOAD_LEN,  RESERVED_LEN, EXPECTED_CMD_ID, HmmdFrame> for HmmdFrame {
    fn new_parser() -> ParserType {
        ParserType::new(CMD_HEADER, CMD_TAIL)
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

