use colored::Colorize;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttrKind {
    CharSeq,
    Number,
    Boolean,
    // Enum(Vec<String>),
    Data,
}
impl AttrKind {
    pub fn from(kind: &str) -> Option<Self> {
        match kind.trim() {
            "string" => Some(AttrKind::CharSeq),
            "number" => Some(AttrKind::Number),
            "boolean" => Some(AttrKind::Boolean),
            "data" => Some(AttrKind::Data),
            _ => None,
        }
    }
}

impl AttrKind {
    pub fn is_boolean(&self) -> bool {
        if let AttrKind::Boolean = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComponentAttr(pub String, pub AttrKind);
impl ComponentAttr {
    pub fn from_key_value(key_value: &str) -> Option<Self> {
        let key_value_splitted = key_value.split("=").collect::<Vec<&str>>();
        let (key, value) = (
            key_value_splitted.get(0)?.trim().to_string(),
            key_value_splitted.get(1)?,
        );
        match AttrKind::from(value) {
            Some(kind) => Some(ComponentAttr(key, kind)),
            None => {
                eprintln!(
                    "{}: the kind of attribute {} doesn't exist, skipping",
                    "warning".yellow().bold(),
                    value
                );
                None
            }
        }
    }
    pub fn from_keys_values(keys_values: &str) -> Vec<Self> {
        keys_values
            .lines()
            .filter_map(ComponentAttr::from_key_value)
            .collect()
    }
}

pub fn to_hashmap(c_attrs: Vec<ComponentAttr>) -> HashMap<String, AttrKind> {
    let mut map = HashMap::new();
    for ComponentAttr(key, val) in c_attrs {
        map.insert(key, val);
    }
    map
}
