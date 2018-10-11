#[derive(Debug)]
pub struct Controller {
    buttons: [bool; 8],
    index: usize,
    strobe: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            buttons: [false; 8],
            index: 0,
            strobe: 0,
        }
    }

    pub fn set_buttons(&mut self, buttons: [bool; 8]) {
        self.buttons = buttons;
    }

    pub fn read(&mut self) -> u8 {
        let value = if self.index < 8 && self.buttons[self.index] {
            1
        } else {
            0
        };
        self.index += 1;
        if self.strobe & 1 == 1 {
            self.index = 0;
        }
        value
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = value;
        if self.strobe & 1 == 1 {
            self.index = 0;
        }
    }
}
