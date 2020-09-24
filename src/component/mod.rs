use anyhow::{Context, Result};
use filepath::FilePath;
use std::{fs, io::Read};

mod attr;

use crate::tags::HTMLTag;
use crate::{
    tags::Tags,
    tags::{Attrs, Tag},
};
use fs::File;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Component {
    name: String,
    attrs: Vec<String>,
    pub body: Tags,
}

impl Component {
    pub fn from_file(file: &mut File) -> Result<Self> {
        let mut raw_file_content = String::new();

        file.read_to_string(&mut raw_file_content)?;

        let file_path = file.path()?;
        let name = file_path
            .file_stem()
            .context("Could not get file name, fatal error")?
            .to_str()
            .context("Could not get file name, fatal error")?;

        Self::from_raw(raw_file_content, name.to_string())
            .context(format!("Error parsing {} component", name))
    }
    pub fn from_raw(raw_body: String, name: String) -> Result<Self> {
        let mut tags = Tag::from_fragment(&raw_body)?;

        match tags.0.get_mut(0) {
            Some(Tag::HTMLTag(tag)) if tag.name == "Component" => Ok(Component {
                name,
                attrs: tag.attrs.0.keys().map(String::from).collect(),
                body: std::mem::take(&mut tag.children),
            }),
            _ => Ok(Component {
                name,
                attrs: vec![],
                body: tags,
            }),
        }
    }
}

impl Component {
    pub fn expand(&self, Attrs(attrs): &Attrs) -> Result<String> {
        let mut raw_body = format!("{}", self.body);

        for (key, val) in attrs {
            raw_body = raw_body.replace(
                &format!("[[{}]]", key),
                match &val {
                    Some(a) => a,
                    None => "true",
                },
            );
        }
        // This solution creates an infinite loop when using nested components and the child
        // component use an attribute with the same name from the parent. I think this solution is one of the best
        // but has this big problem.
        //
        // But, even, if the attributes have different names it would results in an error.
        //
        // while let Some(s) = raw_body.find("[[") {
        //     let e = raw_body.find("]]").context("Could not expand component")? + 2;

        //     let attr_key = &raw_body[(s + 2)..(e - 2)];
        //     let attr_kind = self.attrs.get(attr_key.trim()).context(format!(
        //         "The attribute {} is not defined in component {}",
        //         attr_key, self.name
        //     ))?;

        //     let attr_val = attrs.0.get(attr_key.trim());

        //     match attr_val {
        //         Some(None) if attr_kind.is_boolean() => raw_body.replace_range(s..e, "true"),
        //         Some(None) => {}
        //         Some(Some(a)) => raw_body.replace_range(s..e, dbg!(a)),
        //         None if attr_kind.is_boolean() => raw_body.replace_range(s..e, "false"),
        //         None => {}
        //     };
        // }

        Ok(raw_body)
    }
}
