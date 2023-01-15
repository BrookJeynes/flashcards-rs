pub struct FlashCard {
    pub flipped: bool,
    pub front: String,
    pub back: String,
}

impl FlashCard {
    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
    }
}
