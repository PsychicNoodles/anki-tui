use std::path::{Path, PathBuf};

use anki::{collection::open_collection, i18n::I18n};
use answer::answer;
use clap::{load_yaml, App};
use decks::list_decks;
use dirs::data_dir;
use note::add_note;
use notetype::view_notetypes;
use serde::Serialize;
use slog::{slog_o, Drain, Logger};
use slog_async::OverflowStrategy;
use util::{ApiResult, MessageType};
use view_card::study_card;

mod answer;
mod decks;
mod note;
mod notetype;
mod util;
mod view_card;

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    let base_dir: PathBuf = [
        matches
            .value_of("anki home")
            .map(str::to_owned)
            .unwrap_or_else(|| {
                data_dir()
                    .map(|d| d.join("Anki2"))
                    .expect("could not find application data directory!")
                    .to_str()
                    .expect("could not convert application data directory to string!")
                    .to_owned()
            }),
        matches
            .value_of("profile")
            .expect("profile undefined")
            .to_owned(),
    ]
    .iter()
    .collect();

    let logger = terminal();
    let mut collection = open_collection(
        base_dir.join("collection.anki2"),
        base_dir.join("collection.media"),
        base_dir.join("collection.media.db2"),
        false,
        I18n::new(&["en"], Path::new("en").join("fluent"), logger.clone()),
        logger,
    )
    .expect("collection");

    print_output(
        matches.value_of("output format").expect("output format"),
        match matches.subcommand() {
            ("list-decks", Some(subc)) => list_decks(&mut collection, subc),
            ("study", Some(subc)) => study_card(&mut collection, subc),
            ("answer", Some(subc)) => answer(&mut collection, subc),
            ("add", Some(subc)) => add_note(&mut collection, subc),
            ("note-types", Some(subc)) => view_notetypes(&mut collection, subc),
            _ => Ok(MessageType::Empty),
        },
    );
}

fn print_output(format: &str, output: ApiResult) {
    match output.as_ref() {
        Ok(&MessageType::Empty) => {}
        Ok(msg) => write_format(format, msg).expect("print output"),
        Err(err) => eprintln!("{}", err),
    };
}

fn write_format<T: Serialize>(format: &str, out: &T) -> Result<(), serde_json::Error> {
    match format {
        "pretty-json" => serde_json::to_writer_pretty(std::io::stdout(), out),
        "json" => serde_json::to_writer(std::io::stdout(), out),
        _ => Ok(()),
    }
}

fn terminal() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::new(drain);
    let drain = slog_async::Async::new(drain)
        .chan_size(1_024)
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();
    Logger::root(drain, slog_o!())
}
