use crate::{stateful_list::StatefulList, FlashCard};

use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

pub struct Deck {
    pub title: String,
    pub cards: StatefulList<FlashCard>,
    pub current_card: usize,
}

impl Deck {
    pub fn next(&mut self) {
        if self.current_card < self.cards.items.len() - 1 {
            self.current_card += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.current_card != 0 {
            self.current_card -= 1;
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut irs = Irs::default();

        // Todo: remove unwrap
        irs.shuffle(&mut self.cards.items, &mut rng).unwrap();
    }

    /// Flips all cards in a deck to be hidden
    pub fn hide_all(&mut self) {
        for card in self.cards.items.iter_mut() {
            if card.flipped {
                card.flip();
            }
        }
    }
}
