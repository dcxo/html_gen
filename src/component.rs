use colored::Colorize;
use std::{fs::File, io::Read};

#[derive(Debug, Clone, Eq, PartialEq)]
enum AttrKind {
    CharSeq,
    Number,
    Boolean,
    Enum(Vec<String>),
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Attr(String, AttrKind);

impl Attr {
    pub fn from_key_value(key_value: &str) -> Option<Self> {
        let key_value_splitted = key_value.split("=").collect::<Vec<&str>>();
        let (key, value) = (
            key_value_splitted.get(0)?.trim().to_string(),
            key_value_splitted.get(1)?,
        );
        match AttrKind::from(value) {
            Some(kind) => Some(Attr(key, kind)),
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
            .filter_map(Attr::from_key_value)
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Component {
    attrs: Vec<Attr>,
    pub raw_body: String,
}

impl Component {
    pub fn from_file(file: &mut File) -> Result<Self, ()> {
        let mut raw_file_content = String::new();

        if let Err(_) = file.read_to_string(&mut raw_file_content) {
            return Err(());
        };

        Self::from_raw(raw_file_content)
    }
    pub fn from_raw<S: Into<String>>(raw_body: S) -> Result<Self, ()> {
        let raw_body_transformed = raw_body.into();
        let vec = raw_body_transformed.split("=====").collect::<Vec<&str>>();
        if vec.len() != 2 {
            return Err(());
        }

        let attrs = Attr::from_keys_values(vec.get(0).unwrap());
        let raw_body = vec.get(1).unwrap().trim().to_string();

        Ok(Component { attrs, raw_body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attrs() {
        let l = "id=numer\nname=string\ndata=data\ndisable=boolean";
        let attrs = Attr::from_keys_values(l);

        assert_eq!(
            attrs,
            vec![
                // Attr("id".to_string(), AttrKind::Number),
                Attr("name".to_string(), AttrKind::CharSeq),
                Attr("data".to_string(), AttrKind::Data),
                Attr("disable".to_string(), AttrKind::Boolean)
            ]
        );
    }

    #[test]
    fn component() {
        let rb = r#"name=string

                    age=number
                    desc=data
=====
<h1>Hola</h1>
<p>[[desc]]</p>"#;
        assert_eq!(
            Component::from_raw(rb).unwrap(),
            Component {
                attrs: vec![
                    Attr("name".to_string(), AttrKind::CharSeq),
                    Attr("age".to_string(), AttrKind::Number),
                    Attr("desc".to_string(), AttrKind::Data),
                ],
                raw_body: r#"<h1>Hola</h1>
<p>[[desc]]</p>"#
                    .to_string()
            }
        )
    }
}
