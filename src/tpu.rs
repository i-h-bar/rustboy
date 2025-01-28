pub struct Timer {
    div: u16,
    time: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self{div: 0xAC00, time: 0, tma: 0, tac: 0}
    }

    pub fn tick(&mut self) {}
}