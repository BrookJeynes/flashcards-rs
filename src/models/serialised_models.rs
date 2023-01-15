use serde::Deserialize;

#[derive(Deserialize)]
pub struct SerialisedCard {
    pub front: String,
    pub back: String,
}

#[derive(Deserialize)]
pub struct SerialisedDeck {
    pub title: String,
    pub cards: Vec<SerialisedCard>
}

#[derive(Deserialize)]
pub struct SerialisedDecks {
    pub decks: Vec<SerialisedDeck>
}
