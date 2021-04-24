pub struct WorkState {
    ongoing: u32,
    exiting: bool,
}

impl WorkState {
    pub fn init() -> Self {
        WorkState {
            ongoing: 0,
            exiting: false,
        }
    }

    pub fn ongoing_work(&mut self) {
        self.ongoing += 1
    }

    pub fn done_ongoing_work(&mut self) {
        self.ongoing -= 1
    }

    pub fn nomore_works(&self) -> bool {
        self.ongoing == 0
    }

    pub fn set_exiting(&mut self) {
        self.exiting = true
    }

    pub fn is_exiting(&self) -> bool {
        self.exiting
    }
}
