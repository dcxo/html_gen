use std::{collections::HashMap, fmt::Display};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Attrs(pub HashMap<String, Option<String>>);

fn print_attr((key, val): (&String, &Option<String>)) -> String {
    match &val {
        Some(val) => format!(r#"{}="{}""#, key, val),
        None => format!("{}", key),
    }
}

impl Attrs {
    pub fn from_hashmap(hm: HashMap<String, Option<String>>) -> Self {
        Attrs(hm)
    }
}

impl Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(print_attr)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
