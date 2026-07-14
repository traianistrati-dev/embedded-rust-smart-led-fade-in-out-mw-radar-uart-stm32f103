
#[repr(u16)]
pub enum ParameterID{
    Range = 0x0100,
    Delay = 0x0400,

    TriggerThreshold00 = 0x1000,
    TriggerThreshold01 = 0x1100,
    TriggerThreshold02 = 0x1200,
    TriggerThreshold03 = 0x1300,
    TriggerThreshold04 = 0x1400,
    TriggerThreshold05 = 0x1500,
    TriggerThreshold06 = 0x1600,

    TriggerThreshold07 = 0x1700,
    TriggerThreshold08 = 0x1800,
    TriggerThreshold09 = 0x1900,

    TriggerThreshold10 = 0x1A00,
    TriggerThreshold11 = 0x1B00,
    TriggerThreshold12 = 0x1C00,

    TriggerThreshold13 = 0x1D00,
    TriggerThreshold14 = 0x1E00,
    TriggerThreshold15 = 0x1F00,


    HoldThreshold00 = 0x2000,
    HoldThreshold01 = 0x2100,
    HoldThreshold02 = 0x2200,

    HoldThreshold03 = 0x2300,
    HoldThreshold04 = 0x2400,
    HoldThreshold05 = 0x2500,

    HoldThreshold06 = 0x2600,
    HoldThreshold07 = 0x2700,
    HoldThreshold08 = 0x2800,

    HoldThreshold09 = 0x2900,
    HoldThreshold10 = 0x2A00,
    HoldThreshold11 = 0x2B00,

    HoldThreshold12 = 0x2C00,
    HoldThreshold13 = 0x2D00,
    HoldThreshold14 = 0x2E00,
    HoldThreshold15 = 0x2F00,

}

impl ParameterID{
    pub fn get_bytes(self) -> [u8;2]{
        (self as u16).to_be_bytes()
    }

}

#[repr(u16)]
pub enum CommandID{
    EnableConfig = 0xFF00,
    EnableConfigAck = 0xFF01,
    EndSaveConfig = 0xFE00,
    EndSaveConfigAck = 0xFE01,
    WriteParam = 0x0700,
    WriteParamAck = 0x0701,
    ReadParam = 0x0800,
    ReadParamAck = 0x0801,
    ReadFirmwareVersion = 0x0000,
    ReadSerialNumber = 0x1100,
    ReportMode = 0x1200,//64 00 00 00 = Normal Mode  (ASCII ON/OFF); 04 00 00 00 = Report Mode; 00 00 00 00 = Debug Mode
    None = 0xFFFF,
}

impl CommandID{
    pub fn get_bytes(self) -> [u8;2]{
        (self as u16).to_be_bytes()
    }

    pub const fn raw(self) -> u16 {
        self as u16
    }

}

pub const SEND_HEADER:[u8;4] = [0xFD ,0xFC,0xFB,0xFA];
pub const SEND_TAIL:[u8;4] = [0x04 ,0x03,0x02,0x01];










