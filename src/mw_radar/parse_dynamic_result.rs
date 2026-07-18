enum State {
    Header(usize),
    Length(usize, u16),
    CmdId(usize, u16),
    ReservedBytes(usize),
    Payload(usize),
    Tail(usize),
}

pub trait ParserResult<'a,
const PAYLOAD_LEN: usize,
const RESERVED_LEN: usize,
const EXPECTED_CMD_ID: u16,
RESULT,
> {
    fn new_parser() -> Parser<'a, PAYLOAD_LEN, RESERVED_LEN, EXPECTED_CMD_ID>;
    fn decode(payload: &[u8]) -> RESULT;
}

pub struct Parser<'a,
const PAYLOAD_LEN: usize,
const RESERVED_LEN: usize,
const EXPECTED_CMD_ID: u16,

> {
    state: State,

    pub header: &'a [u8; 4],
    pub tail: &'a [u8; 4],

    pub length: u16,
    pub cmd_id: Option<u16>,
    pub reserved: [u8; RESERVED_LEN],
    pub payload: [u8; PAYLOAD_LEN],
}

impl<'a,
const PAYLOAD_LEN: usize,
const RESERVED_LEN: usize,
const EXPECTED_CMD_ID: u16,

> Parser<'a, PAYLOAD_LEN, RESERVED_LEN, EXPECTED_CMD_ID>
{
    pub const fn new(header: &'a[u8; 4], tail: &'a [u8; 4]) -> Self {
        Self {
            state: State::Header(0),
            header,
            tail,
            length: 0,
            cmd_id: None,
            reserved: [0u8; RESERVED_LEN],
            payload: [0u8; PAYLOAD_LEN],
        }
    }

    #[inline]
    const fn has_cmd_id() -> bool {
        EXPECTED_CMD_ID != super::CommandID::None.raw()
    }
    pub fn clear(&mut self) {
        self.reset();
    }

    fn reset(&mut self) {
        self.state = State::Header(0);
        self.length = 0;
        self.cmd_id = None;
    }

    fn after_length_state() -> State {
        if Self::has_cmd_id() {
            State::CmdId(0, 0)
        } else if RESERVED_LEN > 0 {
            State::ReservedBytes(0)
        } else {
            State::Payload(0)
        }
    }

    fn after_cmd_state() -> State {
        if RESERVED_LEN > 0 {
            State::ReservedBytes(0)
        } else {
            State::Payload(0)
        }
    }

    pub fn feed(&mut self, b: u8) -> bool {
        match self.state {
            State::Header(n) => {
                if b == self.header[n] {
                    self.state = if n == 3 {
                        State::Length(0, 0)
                    } else {
                        State::Header(n + 1)
                    };
                } else {
                    self.state = if b == self.header[0] {
                        State::Header(1)
                    } else {
                        State::Header(0)
                    };
                }
                false
            }

            // LENGTH = little-endian
            State::Length(n, acc) => {
                let acc = acc | ((b as u16) << (n * 8));

                if n == 1 {
                    self.length = acc;

                    let expected_len =
                    PAYLOAD_LEN as u16
                    + RESERVED_LEN as u16
                    + if Self::has_cmd_id() { 2 } else { 0 };

                    if acc == expected_len {
                        self.state = Self::after_length_state();
                    } else {
                        self.reset();
                    }
                } else {
                    self.state = State::Length(1, acc);
                }

                false
            }

            // CMD_ID = big-endian
            // bytes 08 01 => 0x0801
            State::CmdId(n, acc) => {
                let acc = if n == 0 {
                    (b as u16) << 8
                } else {
                    acc | (b as u16)
                };

                if n == 1 {
                    self.cmd_id = Some(acc);

                    if acc != EXPECTED_CMD_ID {
                        self.reset();
                        return false;
                    }

                    self.state = Self::after_cmd_state();
                } else {
                    self.state = State::CmdId(1, acc);
                }

                false
            }

            State::ReservedBytes(n) => {
                self.reserved[n] = b;
                self.state = if n + 1 == RESERVED_LEN {
                    State::Payload(0)
                } else {
                    State::ReservedBytes(n + 1)
                };
                false
            }

            State::Payload(n) => {
                self.payload[n] = b;
                self.state = if n + 1 == PAYLOAD_LEN {
                    State::Tail(0)
                } else {
                    State::Payload(n + 1)
                };
                false
            }

            State::Tail(n) => {
                if b == self.tail[n] {
                    if n == 3 {
                        self.reset();
                        true
                    } else {
                        self.state = State::Tail(n + 1);
                        false
                    }
                } else {
                    self.reset();
                    false
                }
            }
        }
    }
}


pub fn decode_threschold_value(value: u32) -> f32 {
    if value == 0 {
        return 0.0;
    }

    10.0 * libm::log10f(value as f32)
}