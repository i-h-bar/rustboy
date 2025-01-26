pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    JoyPad,
}

const V_BLANK: u8 = 1;
const LCD_STRAT: u8 = 2;
const TIMER: u8 = 4;
const SERIAL: u8 = 8;
const JOY_PAD: u8 = 16;

pub fn fetch_interrupt_num(interrupt: Interrupt) -> u8 {
    match interrupt {
        Interrupt::VBlank => V_BLANK,
        Interrupt::LCDStat => LCD_STRAT,
        Interrupt::Timer => TIMER,
        Interrupt::Serial => SERIAL,
        Interrupt::JoyPad => JOY_PAD,
    }
}
