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

    pub fn tick(&mut self) {
        let previous_div = self.div;
        self.div += 1;

        let mut update_timer = false;

        match self.tac & 0b11 {
            0b00 => {
                if (previous_div & (1 << 9)) != 0 && (!(self.div & (1 << 9))) != 0 {
                    update_timer = true;
                }
            }
            _ => {}
        }
    }
}