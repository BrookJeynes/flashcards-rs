pub mod stateful_list;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use stateful_list::StatefulList;
use std::{error::Error, io::stdout};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

enum Screens {
    DecksView,
    FlashCardView,
}

struct Screen {
    screen_type: Screens,
    current_window: u8,
}

struct FlashCard {
    flipped: bool,
    front: String,
    back: String,
}

impl FlashCard {
    fn flip(&mut self) {
        self.flipped = !self.flipped;
    }
}

struct Deck {
    title: String,
    cards: StatefulList<FlashCard>,
    current_card: usize,
}

impl Deck {
    fn next(&mut self) {
        if self.current_card < self.cards.items.len() - 1 {
            self.current_card += 1;
        }
    }

    fn previous(&mut self) {
        if self.current_card != 0 {
            self.current_card -= 1;
        }
    }
}

struct AppState {
    decks: StatefulList<Deck>,
    current_screen: Screen,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            decks: StatefulList::with_items(vec![
                Deck {
                    title: String::from("German sentences"),
                    cards: StatefulList::with_items(vec![
                        FlashCard {
                            flipped: false,
                            front: String::from("Hey! Wie ist dein Name?"),
                            back: String::from("Hey! What is your name"),
                        },
                        FlashCard {
                            flipped: false,
                            front: String::from(
                                "Bist du online? Wolltest du ein paar Spiele spielen?",
                            ),
                            back: String::from("Are you online? Did you want to play some games?"),
                        },
                    ]),
                    current_card: 0,
                },
                Deck {
                    title: String::from("German computer terms"),
                    cards: StatefulList::with_items(vec![]),
                    current_card: 0,
                },
            ]),
            current_screen: Screen {
                screen_type: Screens::DecksView,
                current_window: 0,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_state = AppState::default();
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
                                    app_state.current_screen.screen_type = Screens::FlashCardView
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

fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();
    let current_window = app_state.current_screen.current_window;

    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    };

    let chunks = Layout::default()
        .margin(1)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(size);

    match app_state.current_screen.screen_type {
        Screens::DecksView => {
            let decks_view = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            let decks: Vec<ListItem> = app_state
                .decks
                .items
                .iter()
                .map(|deck| ListItem::new(deck.title.as_ref()))
                .collect();

            let decks_list = List::new(decks)
                .block(if current_window == 0 {
                    create_block("Decks").border_style(Style::default().fg(Color::Yellow))
                } else {
                    create_block("Decks")
                })
                .highlight_style(Style::default().bg(Color::LightCyan));

            // Render deck list
            f.render_stateful_widget(decks_list, decks_view[0], &mut app_state.decks.state);

            // Render deck cards
            if let Some(index) = app_state.decks.selected() {
                let cards: Vec<ListItem> = app_state.decks.items[index]
                    .cards
                    .items
                    .iter()
                    .enumerate()
                    .map(|(index, card)| {
                        ListItem::new(format!(
                            "{}. {}",
                            index + 1,
                            if !card.flipped {
                                &card.front
                            } else {
                                &card.back
                            }
                        ))
                    })
                    .collect();

                let cards_list = List::new(cards)
                    .block(if current_window == 1 {
                        create_block("Cards").border_style(Style::default().fg(Color::Yellow))
                    } else {
                        create_block("Cards")
                    })
                    .highlight_style(Style::default().bg(Color::LightCyan));

                f.render_stateful_widget(
                    cards_list,
                    decks_view[1],
                    &mut app_state.decks.items[index].cards.state,
                )
            } else {
                f.render_widget(create_block("Cards"), decks_view[1]);
            }

            // Render footer
            let decks_view_footer = Paragraph::new(
                "D: Delete deck | a: Add deck | ENTER: Select deck to practice | SPACE: Flip selected card | TAB: Move to next window | q: Exit application",
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

            f.render_widget(decks_view_footer, chunks[1]);
        }
        Screens::FlashCardView => {
            let current_deck = &app_state.decks.items[app_state
                .decks
                .selected()
                .expect("Deck is already selected at this point")];

            let current_card = &current_deck.cards.items[current_deck.current_card];

            let cards_view = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([
                    Constraint::Percentage(45),
                    Constraint::Percentage(10),
                    Constraint::Percentage(45),
                ])
                .split(chunks[0]);

            let card_front = Paragraph::new(current_card.front.as_ref())
                .alignment(Alignment::Center)
                .block(create_block("Card front").title_alignment(Alignment::Center));

            let card_back = Paragraph::new(if !current_card.flipped {
                "Press SPACE to flip the card"
            } else {
                current_card.back.as_ref()
            })
            .alignment(Alignment::Center)
            .block(create_block("Card back").title_alignment(Alignment::Center));

            f.render_widget(card_front, cards_view[0]);
            f.render_widget(card_back, cards_view[2]);

            // Render footer
            let decks_view_footer = Paragraph::new(
                "SPACE: Reveal card back | h/Left arrow: Previous card | l/Right arrow: Next card  | ESC: Back to decks view | q: Exit application",
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

            f.render_widget(decks_view_footer, chunks[1]);
        }
    }
}
