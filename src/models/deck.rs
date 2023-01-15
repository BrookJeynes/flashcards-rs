use crate::{stateful_list::StatefulList, FlashCard};

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

    pub fn previous(&mut self) { if self.current_card != 0 {
            self.current_card -= 1;
        }
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

