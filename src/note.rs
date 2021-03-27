use anki::{collection::Collection, decks::DeckID, notetype::NoteTypeID};
use clap::ArgMatches;

use crate::util::{ApiResult, Error};

pub fn add_note(collection: &mut Collection, matches: &ArgMatches) -> ApiResult {
    let deck_id = matches
        .value_of("deck id")
        .map(str::parse)
        .map(Result::unwrap)
        .expect("deck id");
    let note_type_raw = matches.value_of("note type").expect("note type");
    let note_type = match note_type_raw.parse() {
        Ok(id) => collection.get_notetype(id),
        Err(_) => collection.get_notetype_by_name(note_type_raw),
    }
    .expect("note type lookup")
    .expect("existing note type");
    let mut new_note = note_type.new_note();

    for (i, field) in matches.values_of("fields").expect("fields").enumerate() {
        new_note
            .set_field(i, field)
            .expect(&format!("setting field {}", i));
    }

    collection
        .add_note(&mut new_note, deck_id)
        .map(From::from)
        .map_err(Error::Anki)
}
