use super::{component::Component, data::get_data};
use regex::{Captures, Regex};
use std::{collections::HashMap, ffi::OsString, ops::Range};

#[derive(Debug)]
pub struct EmptyTags {
    name: String,
    tag: String,
    attrs: Vec<(String, String)>,
}
impl EmptyTags {
    fn from_capture(empty_tag: Captures) -> Option<Self> {
        let tag = empty_tag.get(0)?.as_str();
        let attrs: Vec<(String, String)> = Regex::new(r#"(\w*)="([[[:ascii:]]&&[^"]]*)""#)
            .unwrap()
            .captures_iter(tag)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().to_string(),
                    cap.get(2).unwrap().as_str().to_string(),
                )
            })
            .collect();
        Some(EmptyTags {
            name: empty_tag.get(1)?.as_str().to_string(),
            tag: empty_tag.get(0)?.as_str().to_string(),
            attrs,
        })
    }
}

pub fn get_all_empty_tags(content: &str) -> Vec<EmptyTags> {
    let regex = Regex::new(r"<(\w*).*/>").unwrap();
    regex
        .captures_iter(content)
        .filter_map(EmptyTags::from_capture)
        .collect()
}

pub fn expand_components(
    components_map: HashMap<OsString, Component>,
    raw_body: &mut str,
) -> String {
    let empty_tags = get_all_empty_tags(raw_body);
    let mut cooked_body: String = String::from(raw_body);
    for i in 0..empty_tags.len() {
        let empty_tag = empty_tags.get(i).unwrap();
        cooked_body = cooked_body.replace(
            &empty_tag.tag,
            &components_map
                .get(&OsString::from(&empty_tag.name))
                .unwrap()
                .raw_body,
        );
        for attr in &empty_tag.attrs {
            cooked_body = cooked_body.replace(&format!("[[{}]]", attr.0), &attr.1);
        }
    }
    cooked_body
}

pub fn expand_data(raw_body: &mut str) -> String {
    let regex = Regex::new(r#"\{\{ *([^}]*) *\}\}"#).unwrap();
    let mut cooked_body = String::new();
    cooked_body.push_str(&raw_body);
    for capture in regex.captures_iter(&raw_body) {
        let data_path = capture.get(1).unwrap().as_str();
        let info = get_data(data_path).unwrap();
        cooked_body = cooked_body.replace(capture.get(0).unwrap().as_str(), &info.to_string());
    }
    cooked_body
}
