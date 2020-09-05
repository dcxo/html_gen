use crate::{data, tags::Tags};
use json::JsonValue;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct ForTag {
    repeater: Vec<JsonValue>,
    repeated: Box<Tags>,
}

impl ForTag {
    pub fn new(repeater: Vec<JsonValue>, repeated: Tags) -> Self {
        ForTag {
            repeater,
            repeated: Box::new(repeated),
        }
    }
}

impl Display for ForTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j_val in &self.repeater {
            let mut s = format!("{}", *self.repeated);
            data::expand_data_fragment(&mut s, j_val).unwrap();
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}
