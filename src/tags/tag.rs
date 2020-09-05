use super::{functional::Functional, Attrs};
use crate::component::Component;
use anyhow::{ensure, Context, Result};
use html_parser::{Dom, Element, Node};
use std::{fmt::Display, fs::File, path::PathBuf, vec::IntoIter};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTag {
    pub name: String,
    pub attrs: Attrs,
    pub children: Tags,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ComponentTag {
    component: Box<Component>,
    attrs: Attrs,
    children: Tags,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Text(String),
    HTMLTag(HTMLTag),
    ComponentTag(ComponentTag),
    FunctionalTag(Functional),
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Tags(pub Vec<Tag>);

impl Tag {
    pub fn from_raw(raw_tag: &str) -> Result<Self> {
        let mut dom = Dom::parse(raw_tag)?;
        ensure!(dom.children.len() == 1, "This isn't a raw file");

        // SAFETY: We previously check that dom.children has only one value
        let mut node = unsafe { dom.children.get_unchecked_mut(0) };
        Self::from_node(&mut node)
    }
    fn from_node(node: &mut Node) -> Result<Self> {
        match node {
            Node::Text(text) => Ok(Tag::Text(std::mem::take(text))),
            Node::Element(Element {
                name,
                attributes,
                children,
                ..
            }) => {
                let children = Tags(
                    children
                        .iter_mut()
                        .filter_map(|mut n| Self::from_node(&mut n).ok())
                        .collect(),
                );
                let attrs = Attrs::from_hashmap(std::mem::take(attributes));

                if Functional::is_func(name) {
                    return Ok(Tag::FunctionalTag(Functional::new(name, attrs, children)?));
                }

                let mut path = PathBuf::from("components");
                path.set_file_name(format!("{}.html", name));

                if path.exists() {
                    Ok(Tag::ComponentTag(ComponentTag {
                        component: Box::from(Component::from_file(&mut File::open(path)?)?),
                        attrs,
                        children,
                    }))
                } else {
                    Ok(Tag::HTMLTag(HTMLTag {
                        name: name.to_string(),
                        attrs,
                        children,
                    }))
                }
            }
            Node::Comment(_) => Ok(Tag::Text("".to_string())),
        }
    }
}

impl Tag {
    fn expand_component(&self) -> String {
        if let Tag::ComponentTag(c) = self {
            c.component.expand(&c.attrs).unwrap()
        } else {
            "".to_string()
        }
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag::Text("".to_string())
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::HTMLTag(html_tag) if html_tag.children.0.is_empty() => write!(
                f,
                "<{name} {attrs} />",
                name = html_tag.name,
                attrs = html_tag.attrs
            ),
            Tag::HTMLTag(html_tag) => write!(
                f,
                "<{name} {attrs}>{body}</{name}>",
                name = html_tag.name,
                attrs = html_tag.attrs,
                body = html_tag.children
            ),
            Tag::Text(content) => write!(f, "{}", content),
            Tag::ComponentTag(_) => writeln!(f, "{}", self.expand_component()),
            Tag::FunctionalTag(fortag) => writeln!(f, "{}", fortag),
        }
    }
}

impl<'a> IntoIterator for &'a Tag {
    type Item = &'a Tag;
    type IntoIter = IntoIter<&'a Tag>;
    fn into_iter(self) -> Self::IntoIter {
        fn append<'a>(tag: &'a Tag, v: &mut Vec<&'a Tag>) {
            v.push(tag);
            match tag {
                Tag::HTMLTag(h) => {
                    for tag in &h.children.0 {
                        append(tag, v)
                    }
                }
                Tag::ComponentTag(_) => (),
                _ => (),
            }
        }

        let mut result = vec![];
        append(self, &mut result);
        result.into_iter()
    }
}
