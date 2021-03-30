use anki::notes::Note;
use clap::ArgMatches;

pub mod add;
pub mod edit;

fn set_fields(note: &mut Note, matches: &ArgMatches) {
    for (i, field) in matches.values_of("fields").expect("fields").enumerate() {
        note.set_field(i, field)
            .unwrap_or_else(|_| panic!("setting field {}", i));
    }
}

fn add_tags<'a, T>(note: &mut Note, tags: T)
where
    T: Iterator<Item = &'a str>,
{
    note.tags.append(&mut tags.map(str::to_owned).collect())
}
