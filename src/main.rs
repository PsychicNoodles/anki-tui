use std::path::{Path, PathBuf};

use anki::backend::init_backend;
use anki::backend_proto::BackendService;
use anki::backend_proto::{BackendInit, CloseCollectionIn, OpenCollectionIn};
use bytes::BytesMut;
use clap::{App, Arg, SubCommand};
use dirs::data_dir;
use prost::Message;

mod decks;

fn main() {
    let matches = App::new("anki-tui")
        .version("0.1.0")
        .author("Mattori Birnbaum <mattori.birnbaum@gmail.com>")
        .about("A text interface to Anki")
        .arg(
            Arg::with_name("anki home")
                .short("h")
                .long("home")
                .value_name("FILE")
                .global(true)
                .help("Base directory of Anki config files"),
        )
        .arg(
            Arg::with_name("profile")
                .short("p")
                .long("profile")
                .value_name("PROFILE")
                .global(true)
                .default_value("User 1")
                .help("Anki profile name"),
        )
        .arg(
            Arg::with_name("output-format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .global(true)
                .possible_values(&["pretty-json", "json"])
                .default_value("pretty-json")
                .help("Output serialization format"),
        )
        .subcommand(
            SubCommand::with_name("list-decks")
                .about("Display information about decks")
                .arg(
                    Arg::with_name("deck id")
                        .short("i")
                        .long("deck-id")
                        .value_name("ID")
                        .use_delimiter(true)
                        .help("ID(s) of deck, comma separated"),
                )
                .arg(
                    Arg::with_name("deck name")
                        .short("n")
                        .long("deck-name")
                        .value_name("NAME")
                        .use_delimiter(true)
                        .help("name(s) of deck, comma separated"),
                ),
        )
        .get_matches();

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

    let backend = open_backend(base_dir);

    let output = match matches.subcommand() {
        ("list-decks", Some(subc)) => decks::list_decks(&backend, subc),
        _ => None,
    };
    if let Some(output_val) = output {
        match matches.value_of("output-format").expect("output format") {
            "pretty-json" => serde_json::to_writer_pretty(std::io::stdout(), &output_val),
            "json" => serde_json::to_writer(std::io::stdout(), &output_val),
            _ => Ok(()),
        }
        .expect("output");
    }

    close_backend(backend);
}

fn open_backend(base_dir: PathBuf) -> anki::backend::Backend {
    let backend_init = BackendInit {
        locale_folder_path: Path::new("en")
            .join("fluent")
            .into_os_string()
            .into_string()
            .unwrap(),
        preferred_langs: vec!["en".to_string()],
        server: false,
    };
    let mut buf = BytesMut::with_capacity(1024);
    backend_init.encode(&mut buf).expect("encode");
    let backend = init_backend(&buf).unwrap();
    backend
        .open_collection(OpenCollectionIn {
            collection_path: base_dir
                .join("collection.anki2")
                .into_os_string()
                .into_string()
                .unwrap(),
            media_folder_path: base_dir
                .join("collection.media")
                .into_os_string()
                .into_string()
                .unwrap(),
            media_db_path: base_dir
                .join("collection.media.db2")
                .into_os_string()
                .into_string()
                .unwrap(),
            log_path: base_dir
                .join("collection2.log")
                .into_os_string()
                .into_string()
                .unwrap(),
        })
        .expect("open collection");
    backend
}

fn close_backend(backend: anki::backend::Backend) {
    backend
        .close_collection(CloseCollectionIn {
            downgrade_to_schema11: false,
        })
        .expect("close");
}