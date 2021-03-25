use anki::{
    collection::Collection,
    scheduler::answering::{CardAnswer, Rating},
};
use clap::ArgMatches;

use crate::util::{now_millis, ApiResult, Error};

const ANSWER_VALUES: &str = "[\"again\", \"1\", \"hard\", \"2\", \"good\", \"3\", \"easy\", \"4\"]";

pub fn answer(collection: &mut Collection, matches: &ArgMatches) -> ApiResult {
    let card_id = matches
        .value_of("card id")
        .map(str::parse)
        .map(Result::unwrap)
        .expect("card id");
    let states = collection
        .get_next_card_states(card_id)
        .expect("card states");
    let (new_state, rating) = match matches.value_of("answer").expect("answer") {
        "again" | "1" => (states.again, Rating::Again),
        "hard" | "2" => (states.hard, Rating::Hard),
        "good" | "3" => (states.good, Rating::Good),
        "easy" | "4" => (states.easy, Rating::Easy),
        v => {
            return Err(Error::InvalidParam(
                v.to_string(),
                ANSWER_VALUES.to_string(),
            ))
        }
    };
    let milliseconds_taken = matches
        .value_of("time taken")
        .map(str::parse)
        .map(Result::unwrap)
        .expect("time taken");

    collection
        .answer_card(&CardAnswer {
            card_id,
            current_state: states.current,
            new_state,
            rating,
            answered_at: now_millis(),
            milliseconds_taken,
        })
        .map(From::from)
        .map_err(Error::Anki)
}
