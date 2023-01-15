pub enum Screens {
    DecksView,
    FlashCardView,
}

pub struct Screen {
    pub screen_type: Screens,
    pub current_window: u8,
}
