use super::Attrs;
use crate::component::Component;
use anyhow::{ensure, Result};
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
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Tags(pub Vec<Tag>);

// Constructors
impl Tag {
    pub fn from_raw(raw_tag: &str) -> Result<Self> {
        let dom = Dom::parse(raw_tag)?;
        ensure!(dom.children.len() == 1, "This isn't a raw file");

        // SAFETY: We previously check that dom.children has only one value
        let node = unsafe { dom.children.get_unchecked(0) };
        Ok(Self::from_node(node))
    }
    fn from_node(node: &Node) -> Self {
        match node {
            Node::Text(text) => Tag::Text(text.clone()),
            Node::Element(Element {
                name,
                attributes,
                children,
                ..
            }) => {
                let path = PathBuf::from("components").join(format!("{}.html", name));
                let attrs = Attrs::from_hashmap(attributes);
                let children = Tags(children.iter().map(Self::from_node).collect());
                if path.exists() {
                    Tag::ComponentTag(ComponentTag {
                        attrs,
                        children,
                        component: Box::from(
                            Component::from_file(&mut File::open(path).unwrap()).unwrap(),
                        ),
                    })
                } else {
                    Tag::HTMLTag(HTMLTag {
                        name: name.to_string(),
                        attrs,
                        children,
                    })
                }
            }
            Node::Comment(_) => Tag::Text("".to_string()),
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
            Tag::HTMLTag(html_tag) if html_tag.children.0.is_empty() => {
                write!(f, "<{name} />", name = html_tag.name)
            }
            Tag::HTMLTag(html_tag) => write!(
                f,
                "<{name} {attrs}>{body}</{name}>",
                name = html_tag.name,
                attrs = html_tag.attrs,
                body = html_tag.children
            ),
            Tag::Text(content) => write!(f, "{}", content),
            Tag::ComponentTag(_) => writeln!(f, "{}", self.expand_component()),
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
                Tag::ComponentTag(c) => (),
                _ => (),
            }
        }

        let mut result = vec![];
        append(self, &mut result);
        result.into_iter()
    }
}
