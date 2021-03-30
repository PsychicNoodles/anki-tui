use anki::collection::Collection;
use clap::ArgMatches;

use crate::util::{ApiResult, Error, MessageType};

use super::{add_tags, set_fields};

pub fn edit_card(collection: &mut Collection, matches: &ArgMatches) -> ApiResult {
    let card_id = matches
        .value_of("card id")
        .expect("card id")
        .parse()
        .expect("card id format");

    if let Some(deck_id) = matches.value_of("deck id") {
        collection
            .set_deck(&[card_id], deck_id.parse().expect("deck id"))
            .map_err(Error::Anki)?;
    }

    let card = collection
        .storage
        .get_card(card_id)
        .expect("storage")
        .expect("card");
    let mut note = collection
        .storage
        .get_note(card.note_id)
        .expect("storage")
        .expect("note");

    let mut needs_update = false;

    if let Some(note_type_id) = matches.value_of("note type id") {
        note.notetype_id = note_type_id.parse().expect("note type id");
        needs_update = true;
    }

    if let Some(tags) = matches.values_of("tags") {
        note.tags.clear();
        add_tags(&mut note, tags);
        needs_update = true;
    }

    if let Some(fields) = matches.values_of("fields") {
        set_fields(&mut note, fields);
    }

    if needs_update {
        collection
            .update_note(&mut note)
            .map_err(Error::Anki)
            .expect("update note");
    }

    Ok(MessageType::Empty)
}
