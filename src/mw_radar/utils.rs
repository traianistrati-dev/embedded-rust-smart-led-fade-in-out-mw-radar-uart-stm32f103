use stm32f1xx_hal::pac;

use super::{ParameterID, SerialCmd, ParserResult};


type UsartTxType = stm32f1xx_hal::serial::Tx<pac::USART1>;
type UsartRxType = stm32f1xx_hal::serial::Rx<pac::USART1>;




pub fn decode_threschold_value(value: u32) -> f32 {
    if value == 0 {
        return 0.0;
    }

    10.0 * libm::log10f(value as f32)
}



pub fn send_config(tx: &mut UsartTxType,rx: &mut UsartRxType, max_range:f32, delay_sec:f32, trigger_treschold_00:f32,delay_ms:&impl Fn(u32)){


    begin_config(tx,rx,delay_ms);
    begin_config(tx,rx,delay_ms);

    send_cmd(tx,rx,SerialCmd::new_set_param(ParameterID::Range, max_range),delay_ms);
    send_cmd(tx,rx,SerialCmd::new_set_param(ParameterID::Delay, delay_sec),delay_ms);
    send_cmd(tx,rx,SerialCmd::new_set_param(ParameterID::TriggerThreshold00, trigger_treschold_00),delay_ms);


    send_cmd(tx,rx,SerialCmd::new_set_report_mode(),delay_ms);

    end_save_config(tx,rx,delay_ms);


}

pub fn begin_config(tx: &mut UsartTxType,rx: &mut UsartRxType,delay_ms:&impl Fn(u32)){
    send_cmd(tx,rx,SerialCmd::new_begin_config(),delay_ms);
}

pub fn end_save_config(tx: &mut UsartTxType,rx: &mut UsartRxType,delay_ms:&impl Fn(u32)){
    send_cmd(tx,rx,SerialCmd::new_end_save_config(),delay_ms);
}


pub fn get_param_value<const PAYLOAD_LEN: usize, const RESERVED_LEN: usize, const EXPECTED_CMD_ID: u16>(
    tx: &mut UsartTxType
    ,rx: &mut UsartRxType
    ,param_id:ParameterID
    ,parser:&mut super::Parser<PAYLOAD_LEN,RESERVED_LEN,EXPECTED_CMD_ID>
    ,delay_ms:&impl Fn(u32)
) -> Option<u32>{

    //  send_cmd(tx,rx,SerialCmd::new_begin_config());

    send_cmd_with_result(tx,rx
        ,SerialCmd::send_read_param_value(param_id)
        ,parser
        , super::read_param::ReadParam::decode
        , delay_ms
    )

}


fn send_cmd<const S:usize, const R:usize>(
    tx:   &mut UsartTxType,
    rx:   &mut UsartRxType,
    data:SerialCmd<S,R>,
    delay_ms:&impl Fn(u32)

) {
    for &b in &data.send {
        nb::block!(tx.write(b)).ok();
    }

    delay_ms(data.wait_ms);

    if !data.result.is_empty() {
        let mut result_index = 0;
        loop{

            if let Ok(b) = rx.read(){
                if b != data.result[result_index]{
                    break;
                }

            }else{
                break;
            }

            result_index += 1;

            if result_index == data.result.len(){
                break;
            }
        }
    }
}





fn send_cmd_with_result<const S:usize, const R:usize,const PAYLOAD_LEN: usize, const RESERVED_LEN: usize, const EXPECTED_CMD_ID: u16, RESULT>(
    tx:   &mut UsartTxType,
    rx:   &mut UsartRxType,
    data:SerialCmd<S,R>,
    parser:&mut super::Parser<PAYLOAD_LEN,RESERVED_LEN,EXPECTED_CMD_ID>,
    decoder: fn(&[u8]) -> RESULT,
    delay_ms:&impl Fn(u32)

) -> Option<RESULT>
{
    for &b in &data.send {
        nb::block!(tx.write(b)).ok();
    }

    delay_ms(data.wait_ms);

    parser.clear();

    let mut idle_loops = 0u32;

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

    None

}




pub fn format_text_with_u32_2inline<'a>(
    prefix: &str,
    number: f32,
    prefix_2: &str,
    number_2: f32,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 24];
    let num_str = fmt_f32( number,2, &mut num_buf);

    let mut num_buf_2 = [0u8; 24];
    let num_str_2 = fmt_f32(number_2,2,&mut num_buf_2);
    concat_4(prefix, num_str, prefix_2,num_str_2, out)
}

pub fn concat_4<'a>(
    a: &str,
    b: &str,
    c: &str,
    d: &str,
    out: &'a mut [u8],
) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let c_bytes = c.as_bytes();
    let d_bytes = d.as_bytes();

    let total_len = a_bytes.len() + b_bytes.len() + c_bytes.len()+ d_bytes.len();

    if total_len > out.len() {
        return "-";
    }

    let mut pos = 0;

    out[pos..pos + a_bytes.len()].copy_from_slice(a_bytes);
    pos += a_bytes.len();

    out[pos..pos + b_bytes.len()].copy_from_slice(b_bytes);
    pos += b_bytes.len();

    out[pos..pos + c_bytes.len()].copy_from_slice(c_bytes);
    pos += c_bytes.len();

    out[pos..pos + d_bytes.len()].copy_from_slice(d_bytes);
    pos += d_bytes.len();

    core::str::from_utf8(&out[..pos]).unwrap_or("-")
}





pub fn format_text_with_u32<'a>(
    prefix: &str,
    number: u32,
    suffix: &str,
    out: &'a mut [u8],
) -> &'a str {
    let mut num_buf = [0u8; 10];
    let num_str = fmt_u32(number, &mut num_buf);
    concat_3(prefix, num_str, suffix, out)
}





pub fn concat_3<'a>(
    a: &str,
    b: &str,
    c: &str,
    out: &'a mut [u8],
) -> &'a str {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let c_bytes = c.as_bytes();

    let total_len = a_bytes.len() + b_bytes.len() + c_bytes.len();

    if total_len > out.len() {
        return "-";
    }

    let mut pos = 0;

    out[pos..pos + a_bytes.len()].copy_from_slice(a_bytes);
    pos += a_bytes.len();

    out[pos..pos + b_bytes.len()].copy_from_slice(b_bytes);
    pos += b_bytes.len();

    out[pos..pos + c_bytes.len()].copy_from_slice(c_bytes);
    pos += c_bytes.len();

    core::str::from_utf8(&out[..pos]).unwrap_or("-")
}




pub fn fmt_u32(mut n: u32, buf: &mut [u8; 10]) -> &str {
    if n == 0 {
        buf[0] = b'0';
        return core::str::from_utf8(&buf[..1]).unwrap();
    }

    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    core::str::from_utf8(&buf[i..]).unwrap()
}

pub fn fmt_u16(n: u16, buf: &mut [u8; 10]) -> &str {
    fmt_u32(n as u32, buf)
}




pub fn fmt_f32(value: f32, decimals: u32, buf: &mut [u8; 24]) -> &str {
    let mut pos = 0usize;
    let mut v = value;

    if v.is_sign_negative() {
        buf[pos] = b'-';
        pos += 1;
        v = -v;
    }

    let scale = match decimals {
        0 => 1u32,
        1 => 10u32,
        2 => 100u32,
        3 => 1000u32,
        4 => 10000u32,
        _ => 100u32, // fallback
    };

    let scaled = (v * scale as f32 + 0.5) as u32;
    let int_part = scaled / scale;
    let frac_part = scaled % scale;

    // partea întreagă
    let mut int_buf = [0u8; 10];
    let int_str = fmt_u32(int_part, &mut int_buf);
    let int_bytes = int_str.as_bytes();

    buf[pos..pos + int_bytes.len()].copy_from_slice(int_bytes);
    pos += int_bytes.len();

    if decimals > 0 {
        buf[pos] = b'.';
        pos += 1;

        // scriem fracția cu zero-padding
        let mut div = scale / 10;
        while div > 0 {
            buf[pos] = b'0' + ((frac_part / div) % 10) as u8;
            pos += 1;
            div /= 10;
        }
    }

    core::str::from_utf8(&buf[..pos]).unwrap()
}


