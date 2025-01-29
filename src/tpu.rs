pub struct Timer {
    div: u16,
    time: u8,
    tma: u8,
    tac: u8,
    ticks: u64,
}

impl Timer {
    pub fn new() -> Self {
        Self{div: 0xAC00, time: 0, tma: 0, tac: 0, ticks: 0}
    }

    pub fn emu_cycles(&mut self, n: u8) {
        for _ in 0..(n * 4) {
            self.ticks += 1;
            self.tick()
        }
    }

    fn tick(&mut self) {
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