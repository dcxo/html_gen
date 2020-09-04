use super::{Attrs, Tags};
use crate::data;
use anyhow::{ensure, Context, Result};
use fortag::ForTag;
use json::JsonValue;
use std::fmt::Display;

mod fortag;

/// Enum to recolect all functional tags html_gen support (e.g. for tag)
#[derive(Debug, PartialEq, Clone)]
pub enum Functional {
    For(ForTag),
}

impl Functional {
    pub fn new(name: &str, attributes: Attrs, children: Tags) -> Result<Functional> {
        match name {
            "for" => Self::new_for(attributes, children),
            _ => unreachable!(),
        }
    }
    fn new_for(Attrs(attributes): Attrs, repeated: Tags) -> Result<Functional> {
        let each = attributes
            .get("each")
            .context("each attribute is not defined for")?
            .as_ref()
            .context("Attribute `each` can't be empty")?;

        let d = data::get_data(&each[2..each.len() - 2])?;

        ensure!(d.is_array(), "The for's data should be an array");

        if let JsonValue::Array(repeater) = d {
            return Ok(Functional::For(ForTag::new(repeater, repeated)));
        }
        unreachable!()
    }
}

impl Functional {
    pub fn is_func(tag_name: &str) -> bool {
        match tag_name {
            "for" => true,
            _ => false,
        }
    }
}

impl Display for Functional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Functional::For(fortag) => write!(f, "{}", fortag),
        }
    }
}
