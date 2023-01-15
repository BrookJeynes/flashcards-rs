use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{AppState, Screens};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
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
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

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
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                f.render_stateful_widget(
                    cards_list,
                    decks_view[1],
                    &mut app_state.decks.items[index].cards.state,
                )
            } else {
                f.render_widget(
                    if current_window == 1 {
                        create_block("Cards").border_style(Style::default().fg(Color::Yellow))
                    } else {
                        create_block("Cards")
                    },
                    decks_view[1],
                );
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
                .block(create_block("Card front").title_alignment(Alignment::Center))
                .wrap(Wrap { trim: true });

            let card_back = Paragraph::new(if !current_card.flipped {
                "Press SPACE to flip the card"
            } else {
                current_card.back.as_ref()
            })
            .alignment(Alignment::Center)
            .block(create_block("Card back").title_alignment(Alignment::Center))
            .wrap(Wrap { trim: true });

            f.render_widget(card_front, cards_view[0]);
            f.render_widget(card_back, cards_view[2]);

            // Render footer
            let decks_view_footer = Paragraph::new(
                "SPACE: Reveal card back | h/Left arrow: Previous card | l/Right arrow: Next card | r: Randomise cards | ESC: Back to decks view | q: Exit application",
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

            f.render_widget(decks_view_footer, chunks[1]);
        }
    }
}
