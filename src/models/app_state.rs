use crate::{stateful_list::StatefulList, Screen, Screens, Deck};

pub struct AppState {
    pub decks: StatefulList<Deck>,
    pub current_screen: Screen,
}

impl AppState {
    pub fn new(decks: Vec<Deck>) -> Self {
        Self {
            decks: StatefulList::with_items(decks),
            current_screen: Screen {
                screen_type: Screens::DecksView,
                current_window: 0,
            },
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            decks: StatefulList::with_items(vec![]),
            current_screen: Screen {
                screen_type: Screens::DecksView,
                current_window: 0,
            },
        }
    }
}
