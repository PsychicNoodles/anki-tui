use std::convert::{TryFrom, TryInto};

use anki::collection::Collection;
use clap::ArgMatches;
use serde::Serialize;

use crate::util::{ApiResult, Error};

#[derive(Serialize)]
pub struct NoteType {
    id: i64,
    name: String,
    mtime: i64,
    fields: Vec<NoteField>,
    templates: Vec<CardTemplate>,
    config: NoteTypeConfig,
}

impl TryFrom<&anki::notetype::NoteType> for NoteType {
    type Error = Error;

    fn try_from(v: &anki::notetype::NoteType) -> Result<Self, Self::Error> {
        Ok(NoteType {
            id: v.id.0,
            name: v.name.clone(),
            mtime: v.mtime_secs.0,
            fields: v.fields.iter().map(From::from).collect(),
            templates: v.templates.iter().map(From::from).collect(),
            config: TryFrom::try_from(&v.config)?,
        })
    }
}

#[derive(Serialize)]
pub struct NoteField {
    name: String,
    sticky: bool,
    rtl: bool,
    font_name: String,
    font_size: u32,
}

impl From<&anki::notetype::NoteField> for NoteField {
    fn from(v: &anki::notetype::NoteField) -> Self {
        NoteField {
            name: v.name.clone(),
            sticky: v.config.sticky,
            rtl: v.config.rtl,
            font_name: v.config.font_name.clone(),
            font_size: v.config.font_size,
        }
    }
}

#[derive(Serialize)]
pub struct CardTemplate {
    name: String,
    mtime: i64,
    q_format: String,
    a_format: String,
    target_deck_id: i64,
}

impl From<&anki::notetype::CardTemplate> for CardTemplate {
    fn from(v: &anki::notetype::CardTemplate) -> Self {
        CardTemplate {
            name: v.name.clone(),
            mtime: v.mtime_secs.0,
            q_format: v.config.q_format.clone(),
            a_format: v.config.a_format.clone(),
            target_deck_id: v.config.target_deck_id,
        }
    }
}

#[derive(Serialize)]
pub struct NoteTypeConfig {
    kind: NoteKind,
    sort_field_idx: u32,
    css: String,
    target_deck_id: i64,
    latex_pre: String,
    latex_post: String,
    latex_svg: bool,
    reqs: Vec<CardRequirement>,
}

impl TryFrom<&anki::notetype::NoteTypeConfig> for NoteTypeConfig {
    type Error = Error;

    fn try_from(value: &anki::notetype::NoteTypeConfig) -> Result<Self, Self::Error> {
        Ok(NoteTypeConfig {
            kind: value.kind.try_into()?,
            sort_field_idx: value.sort_field_idx,
            css: value.css.clone(),
            target_deck_id: value.target_deck_id,
            latex_pre: value.latex_pre.clone(),
            latex_post: value.latex_post.clone(),
            latex_svg: value.latex_svg,
            reqs: value
                .reqs
                .iter()
                .map(TryFrom::try_from)
                .map(Result::unwrap)
                .collect(),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteKind {
    Normal,
    Cloze,
}

impl TryFrom<i32> for NoteKind {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NoteKind::Normal),
            1 => Ok(NoteKind::Cloze),
            _ => Err(Error::InvalidNumber(value.to_string(), "0, 1".to_string())),
        }
    }
}

#[derive(Serialize)]
pub struct CardRequirement {
    kind: CardRequirementKind,
    card_ord: u32,
    field_ords: Vec<u32>,
}

impl TryFrom<&anki::notetype::CardRequirement> for CardRequirement {
    type Error = Error;

    fn try_from(v: &anki::notetype::CardRequirement) -> Result<Self, Self::Error> {
        Ok(CardRequirement {
            kind: TryFrom::try_from(v.kind)?,
            card_ord: v.card_ord,
            field_ords: v.field_ords.clone(),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CardRequirementKind {
    None,
    Any,
    All,
}

impl TryFrom<i32> for CardRequirementKind {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CardRequirementKind::None),
            1 => Ok(CardRequirementKind::Any),
            2 => Ok(CardRequirementKind::All),
            _ => Err(Error::InvalidNumber(
                value.to_string(),
                "0, 1, 2".to_string(),
            )),
        }
    }
}

pub fn view_notetypes(collection: &mut Collection, matches: &ArgMatches) -> ApiResult {
    if let Some(ids) = matches.values_of("note ids") {
        let res: Vec<NoteType> = ids
            .map(|id| id.parse().map(|ntid| collection.get_notetype(ntid)))
            .flatten()
            .flatten()
            .flatten()
            .map(|arc| NoteType::try_from(arc.as_ref()))
            .map(Result::unwrap)
            .collect();
        if res.len() > 0 {
            Ok(From::from(res))
        } else {
            Err(Error::NoResults)
        }
    } else {
        collection
            .get_all_notetypes()
            .map(|nts| -> Vec<NoteType> {
                nts.values()
                    .map(|arc| arc.as_ref())
                    .map(TryFrom::try_from)
                    .map(Result::unwrap)
                    .collect()
            })
            .map(From::from)
            .map_err(Error::Anki)
    }
}
