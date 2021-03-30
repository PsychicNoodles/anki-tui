use anki::notes::Note;

pub mod add;
pub mod edit;

fn set_fields<'a, T>(note: &mut Note, fields: T)
where
    T: Iterator<Item = &'a str>,
{
    for (i, field) in fields.enumerate() {
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
