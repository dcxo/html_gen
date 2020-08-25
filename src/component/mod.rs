use anyhow::{ensure, Result};
use std::{fs, io::Read};

mod attr;

use crate::tag::{Attrs, Tag};
use attr::ComponentAttr;
use fs::File;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Component {
    attrs: Vec<ComponentAttr>,
    pub body: Tag,
}

impl Component {
    pub fn from_file(file: &mut File) -> Result<Self> {
        let mut raw_file_content = String::new();

        file.read_to_string(&mut raw_file_content)?;

        Self::from_raw(raw_file_content)
    }
    pub fn from_raw<S: Into<String>>(raw_body: S) -> Result<Self> {
        let raw_body_transformed = raw_body.into();
        let vec = raw_body_transformed.split("=====").collect::<Vec<&str>>();

        ensure!(vec.len() == 2, "Error parsing a component");

        let attrs = ComponentAttr::from_keys_values(vec.get(0).unwrap());
        let raw_body = unsafe { vec.get_unchecked(1) };

        Ok(Component {
            attrs,
            body: Tag::from_raw(raw_body)?,
        })
    }
}

impl Component {
    pub fn expand(&self, attrs: &Attrs) -> Result<String> {
        let mut idkwtda = format!("{}", self.body);

        for attr in &attrs.0 {
            idkwtda = idkwtda.replace(
                &format!("[[{}]]", attr.0),
                match &attr.1 {
                    Some(a) => a,
                    _ => unreachable!(),
                }
                .as_str(),
            );
        }

        Ok(idkwtda)
    }
}
