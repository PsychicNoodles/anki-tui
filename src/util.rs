use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use anki::{
    prelude::AnkiError,
    timestamp::{TimestampMillis, TimestampSecs},
};
use derive_more::From;
use serde::Serialize;

use thiserror::Error;

use crate::{decks::Deck, notetype::NoteType, view_card::Card};

pub fn now_secs() -> TimestampSecs {
    TimestampSecs(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs() as i64,
    )
}

pub fn now_millis() -> TimestampMillis {
    TimestampMillis(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis() as i64,
    )
}

#[derive(Serialize)]
pub struct Output {
    pub status: Status,
    pub message: MessageType,
}

impl<E: Display> From<Result<MessageType, E>> for Output {
    fn from(r: Result<MessageType, E>) -> Self {
        match r {
            Ok(message) => Output {
                status: Status::Success,
                message,
            },
            Err(err) => Output {
                status: Status::Error,
                message: MessageType::Message(err.to_string()),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
}

#[derive(Serialize, From)]
#[serde(untagged)]
pub enum MessageType {
    Decks(Vec<Deck>),
    Card(Card),
    NoteTypes(Vec<NoteType>),
    Empty,
    Message(String),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("anki error: {0}")]
    Anki(AnkiError),
    #[error("invalid parameter `{0}`, must be `{1}`")]
    InvalidParam(String, String),
    #[error("no results found")]
    NoResults,
    #[error("invalid number format `{0}`, must be in `[{1}]`")]
    InvalidNumber(String, String),
}

pub type ApiResult = std::result::Result<MessageType, Error>;
