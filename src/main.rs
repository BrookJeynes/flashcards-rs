pub mod models;
pub mod stateful_list;
pub mod ui;

use models::{
    app_state::AppState,
    deck::Deck,
    flashcard::FlashCard,
    screen::{Screen, Screens},
    serialised_models,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serialised_models::SerialisedDecks;
use stateful_list::StatefulList;
use std::{error::Error, fs, io::stdout};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

fn read_from_file() -> Result<Vec<Deck>, Box<dyn Error>> {
    let json_decks_string = fs::read_to_string("decks.json")?;
    let json_decks: SerialisedDecks = serde_json::from_str(&json_decks_string)?;

    let decks: Vec<Deck> = json_decks
        .decks
        .iter()
        .map(|deck| {
            let cards: Vec<FlashCard> = deck
                .cards
                .iter()
                .map(|card| FlashCard {
                    flipped: false,
                    front: card.front.to_string(),
                    back: card.back.to_string(),
                })
                .collect();

            Deck {
                title: deck.title.to_string(),
                cards: StatefulList::with_items(cards),
                current_card: 0,
            }
        })
        .collect();

    Ok(decks)
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let decks = match read_from_file() {
        Ok(decks) => Some(decks),
        Err(_) => None,
    };

    let app_state = if let Some(decks) = decks {
        AppState::new(decks)
    } else {
        AppState::default()
    };

    let res = run_app(&mut terminal, app_state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            match app_state.current_screen.screen_type {
                Screens::DecksView => match key.code {
                    // Exit keys
                    KeyCode::Char('q') => return Ok(()),

                    // Deck interaction keys
                    KeyCode::Char('D') => {
                        let window_index = app_state.current_screen.current_window;

                        let deck_index = if let Some(deck_index) = app_state.decks.selected() {
                            deck_index
                        } else {
                            continue;
                        };

                        match (
                            window_index,
                            app_state.decks.items[deck_index].cards.selected(),
                        ) {
                            (0, _) => app_state.decks.delete(deck_index),
                            (1, Some(card_index)) => {
                                app_state.decks.items[deck_index].cards.delete(card_index)
                            }
                            _ => {}
                        }
                    }

                    // Card interaction keys
                    KeyCode::Enter => {
                        let window_index = app_state.current_screen.current_window;

                        let deck_index = if let Some(deck_index) = app_state.decks.selected() {
                            deck_index
                        } else {
                            continue;
                        };

                        match (
                            window_index,
                            app_state.decks.items[deck_index].cards.selected(),
                        ) {
                            (0, _) => {
                                if !app_state.decks.items[deck_index].cards.items.is_empty() {
                                    app_state.current_screen.screen_type = Screens::FlashCardView;
                                    app_state.decks.items[deck_index].hide_all();
                                }
                            }

                            _ => {}
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(deck_index) = app_state.decks.selected() {
                            let window_index = app_state.current_screen.current_window;

                            match (
                                window_index,
                                app_state.decks.items[deck_index].cards.selected(),
                            ) {
                                (1, Some(card_index)) => {
                                    app_state.decks.items[deck_index].cards.items[card_index]
                                        .flip();
                                }

                                _ => {}
                            }
                        }
                    }

                    // Window movement
                    KeyCode::Tab => {
                        let window_index = app_state.current_screen.current_window;

                        app_state.current_screen.current_window = (window_index + 1) % 2;
                    }

                    // Vertical movement keys
                    KeyCode::Char('k') | KeyCode::Up => {
                        let window_index = app_state.current_screen.current_window;

                        match (window_index, app_state.decks.selected()) {
                            (0, _) => app_state.decks.previous(),
                            (1, Some(deck_index)) => {
                                app_state.decks.items[deck_index].cards.previous()
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        let window_index = app_state.current_screen.current_window;

                        match (window_index, app_state.decks.selected()) {
                            (0, _) => app_state.decks.next(),
                            (1, Some(deck_index)) => app_state.decks.items[deck_index].cards.next(),
                            _ => {}
                        }
                    }

                    _ => {}
                },
                Screens::FlashCardView => match key.code {
                    // Exit keys
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => app_state.current_screen.screen_type = Screens::DecksView,

                    // Card interaction keys
                    KeyCode::Char('l') | KeyCode::Right => {
                        let selected_deck = app_state
                            .decks
                            .selected()
                            .expect("Deck is already selected by this point");

                        app_state.decks.items[selected_deck].next();
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        let selected_deck = app_state
                            .decks
                            .selected()
                            .expect("Deck is already selected by this point");

                        app_state.decks.items[selected_deck].previous();
                    }
                    KeyCode::Char(' ') => {
                        if let Some(deck_index) = app_state.decks.selected() {
                            let card_index = app_state.decks.items[deck_index].current_card;

                            app_state.decks.items[deck_index].cards.items[card_index].flip();
                        }
                    }

                    _ => {}
                },
            }
        }
    }
}
