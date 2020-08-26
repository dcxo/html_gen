use anyhow::{ensure, Context, Result};
use filepath::FilePath;
use std::{collections::HashMap, fs, io::Read};

mod attr;

use crate::tag::{Attrs, Tag};
use attr::{AttrKind, ComponentAttr};
use fs::File;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Component {
    name: String,
    attrs: HashMap<String, AttrKind>,
    pub body: Tag,
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
        let vec = raw_body.split("=====").collect::<Vec<&str>>();

        ensure!(vec.len() == 2, "Error parsing a component");

        let attrs = ComponentAttr::from_keys_values(vec.get(0).unwrap());
        let raw_body = unsafe { vec.get_unchecked(1) };

        Ok(Component {
            name,
            attrs: attr::to_hashmap(attrs),
            body: Tag::from_raw(raw_body)?,
        })
    }
}

impl Component {
    pub fn expand(&self, attrs: &Attrs) -> Result<String> {
        let mut raw_body = format!("{}", self.body);

        for attr in &attrs.0 {
            raw_body = raw_body.replace(
                &format!("[[{}]]", attr.0),
                match &attr.1 {
                    Some(a) => a,
                    _ => unreachable!(),
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
