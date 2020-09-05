use super::{Attrs, Tags};
use crate::data;
use anyhow::{ensure, Context, Result};
use fortag::ForTag;
use json::JsonValue;
use sasstag::SassTag;
use std::fmt::Display;

mod fortag;
mod sasstag;

/// Enum to recolect all functional tags html_gen support (e.g. for tag)
#[derive(Debug, PartialEq, Clone)]
pub enum Functional {
    For(ForTag),
    Sass(SassTag),
}

impl Functional {
    pub fn new(name: &str, attributes: Attrs, children: Tags) -> Result<Functional> {
        match name {
            "for" => Self::new_for(attributes, children),
            "sass" | "scss" => Self::new_sass(attributes),
            _ => unreachable!(),
        }
    }
    fn new_for(Attrs(attributes): Attrs, repeated: Tags) -> Result<Functional> {
        let each = attributes
            .get("each")
            .context("`each` attribute must be defined for tag for")?
            .as_ref()
            .context("`each` attribute can't be empty")?;

        let d = data::get_data(&each[2..each.len() - 2])?;

        ensure!(d.is_array(), "The for's data should be an array");

        if let JsonValue::Array(repeater) = d {
            return Ok(Functional::For(ForTag::new(repeater, repeated)));
        }
        unreachable!()
    }
    fn new_sass(Attrs(attributes): Attrs) -> Result<Functional> {
        let src_attr = attributes
            .get("src")
            .context("`src` attribute must be defined for tag scss/sass")?
            .as_ref()
            .context("`src` atribbute can't be empty")?;

        Ok(Functional::Sass(SassTag::new(src_attr)?))
    }
}

impl Functional {
    pub fn is_func(tag_name: &str) -> bool {
        match tag_name {
            "for" => true,
            "sass" | "scss" => true,
            _ => false,
        }
    }
}

impl Display for Functional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Functional::For(fortag) => write!(f, "{}", fortag),
            Functional::Sass(sasstag) => write!(f, "{}", sasstag),
        }
    }
}
