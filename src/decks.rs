use std::{
    collections::{HashSet, VecDeque},
    time::{SystemTime, UNIX_EPOCH},
};

use anki::{backend_proto::DeckTreeNode, collection::Collection, timestamp::TimestampSecs};
use clap::ArgMatches;
use serde::Serialize;

#[derive(Serialize, Default, PartialEq, Eq, Hash)]
pub struct Deck {
    id: i64,
    name: String,
    level: u32,
    collapsed: bool,
    review_count: u32,
    learn_count: u32,
    new_count: u32,
    filtered: bool,
}

impl From<DeckTreeNode> for Deck {
    fn from(n: DeckTreeNode) -> Self {
        Deck {
            id: n.deck_id,
            name: n.name,
            level: n.level,
            collapsed: n.collapsed,
            review_count: n.review_count,
            learn_count: n.learn_count,
            new_count: n.new_count,
            filtered: n.filtered,
        }
    }
}

pub fn list_decks(collection: &mut Collection, matches: &ArgMatches) -> Option<Vec<Deck>> {
    let filter_ids: HashSet<i64> = matches
        .values_of("deck id")
        .map(|vals| vals.map(str::parse).flatten().collect())
        .unwrap_or_default();

    let filter_names: HashSet<&str> = matches
        .values_of("deck name")
        .map(|vals| vals.collect())
        .unwrap_or_default();

    let deck_tree = collection
        .deck_tree(
            Some(TimestampSecs(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_secs() as i64,
            )),
            None,
        )
        .expect("deck tree");

    let decks = DeckTreeNodeIter::from(deck_tree)
        .filter(|n| n.deck_id != 0)
        .map(Deck::from);

    Some(
        decks
            .filter(|d| filter_ids.is_empty() || filter_ids.contains(&d.id))
            .filter(|d| filter_names.is_empty() || filter_names.contains(&d.name.as_str()))
            .into_iter()
            .collect::<Vec<_>>(),
    )
}

#[derive(Default)]
struct DeckTreeNodeIter {
    children: VecDeque<DeckTreeNode>,
    parent: Option<Box<DeckTreeNodeIter>>,
}

impl Iterator for DeckTreeNodeIter {
    type Item = DeckTreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut child) = self.children.pop_front() {
            *self = DeckTreeNodeIter {
                children: VecDeque::from(
                    child
                        .children
                        .drain(..child.children.len())
                        .collect::<Vec<_>>(),
                ),
                parent: Some(Box::new(std::mem::take(self))),
            };
            // println!("child {:?}", &child);
            Some(child)
        } else if let Some(parent) = self.parent.take() {
            *self = *parent;
            self.next()
        } else {
            None
        }
    }
}

impl From<DeckTreeNode> for DeckTreeNodeIter {
    fn from(n: DeckTreeNode) -> DeckTreeNodeIter {
        let mut children = VecDeque::new();
        children.push_back(n);
        DeckTreeNodeIter {
            children,
            parent: None,
        }
    }
}
