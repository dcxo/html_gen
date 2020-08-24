use std::{collections::HashMap, fmt::Display};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Attr(pub String, pub Option<String>);

impl Attr {
    pub fn from_tuple(t: (&String, &Option<String>)) -> Self {
        Attr(t.0.to_string(), t.1.clone())
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Attrs(pub Vec<Attr>);

impl Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.1 {
            Some(val) => write!(f, r#"{}="{}""#, self.0, val),
            None => write!(f, "{}", self.0),
        }
    }
}

impl Attrs {
    pub fn from_hashmap(hm: &HashMap<String, Option<String>>) -> Self {
        Attrs(hm.iter().map(Attr::from_tuple).collect())
    }
}

impl Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|attr| format!("{}", attr))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
