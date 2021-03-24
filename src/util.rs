use std::time::{SystemTime, UNIX_EPOCH};

use anki::timestamp::TimestampSecs;
use derive_more::From;
use serde::Serialize;

use crate::{decks::Deck, review::Card};

pub fn now() -> TimestampSecs {
    return TimestampSecs(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs() as i64,
    );
}

#[derive(Serialize)]
pub struct Output {
    pub status: i64,
    #[serde(flatten)]
    pub message: MessageType,
}

#[derive(Serialize, From)]
pub enum MessageType {
    Decks(Vec<Deck>),
    Card(Card),
}
