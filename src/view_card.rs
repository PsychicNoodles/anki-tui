use std::convert::TryFrom;

use anki::{backend::Backend, collection::Collection, prelude::AnkiError, template::RenderedNode};
use clap::ArgMatches;
use serde::Serialize;

use crate::util::{now_secs, ApiResult, Output};

#[derive(Serialize)]
pub struct Card {
    id: i64,
    back: Vec<CardContent>,
    front: Vec<CardContent>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CardContent {
    Text {
        text: String,
    },
    Replacement {
        field_name: String,
        current_text: String,
        filters: Vec<String>,
    },
}

impl From<RenderedNode> for CardContent {
    fn from(rn: RenderedNode) -> Self {
        match rn {
            RenderedNode::Text { text } => CardContent::Text { text },
            RenderedNode::Replacement {
                field_name,
                current_text,
                filters,
            } => CardContent::Replacement {
                field_name,
                current_text,
                filters,
            },
        }
    }
}

// todo counts

pub fn study_card(collection: &mut Collection, matches: &ArgMatches) -> ApiResult {
    if let Some(deck_id) = matches
        .value_of("deck id")
        .map(str::parse)
        .map(Result::unwrap)
    {
        collection
            .set_current_deck_id(deck_id)
            .expect("set current deck");
    }

    let mut card = next_card(collection).expect("next card");

    match matches.value_of("side").expect("side") {
        "back" | "question" => card.front.clear(),
        "front" | "answer" => card.back.clear(),
        _ => {}
    };

    Ok(From::from(card))
}

fn next_card(collection: &mut Collection) -> Result<Card, AnkiError> {
    let next = collection
        .get_queues()?
        .next_entry(now_secs())
        .expect("next card");
    collection
        .render_existing_card(next.id, false)
        .map(|card| Card {
            id: From::from(next.id),
            back: card.qnodes.into_iter().map(From::from).collect(),
            front: card.anodes.into_iter().map(From::from).collect(),
        })
}
