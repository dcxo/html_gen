mod attr;
mod tag;

pub use attr::{Attr, Attrs};
pub use tag::{HTMLTag, Tag, Tags};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_tag_iter() {
        let tags = Tag::HTMLTag(HTMLTag {
            name: "html".to_string(),
            attrs: Attrs(vec![Attr(String::from("lang"), Some(String::from("es")))]),
            children: Tags(vec![
                Tag::HTMLTag(HTMLTag {
                    name: "head".to_string(),
                    ..Default::default()
                }),
                Tag::HTMLTag(HTMLTag {
                    name: "body".to_string(),
                    attrs: Attrs(vec![]),
                    children: Tags(vec![Tag::HTMLTag(HTMLTag {
                        name: "h1".to_string(),
                        children: Tags(vec![
                            Tag::Text("Hola Mundo!".to_string()),
                            Tag::HTMLTag(HTMLTag {
                                name: "span".to_string(),
                                children: Tags(vec![Tag::Text(
                                    "It is me, the Great Papyrus".to_string(),
                                )]),
                                ..Default::default()
                            }),
                        ]),
                        ..Default::default()
                    })]),
                }),
            ]),
        });
    }
    #[test]
    fn parse_html() {
        let body = r#"<!DOCTYPE html>
<   html>
    <head   >
        <meta key="h" content="w" />
    </head>
    <body>
        <h1>hola {{ data }} adios</ h1    >
    </body>
</html>"#;

        let parsed = Tag::from_raw(body).unwrap();

        assert_eq!(
            parsed,
            Tag::HTMLTag(HTMLTag {
                name: "html".to_string(),
                children: Tags(vec![
                    Tag::HTMLTag(HTMLTag {
                        name: "head".to_string(),
                        children: Tags(vec![Tag::HTMLTag(HTMLTag {
                            name: "meta".to_string(),
                            ..Default::default()
                        })]),
                        ..Default::default()
                    }),
                    Tag::HTMLTag(HTMLTag {
                        name: "body".to_string(),
                        children: Tags(vec![Tag::HTMLTag(HTMLTag {
                            name: "h1".to_string(),
                            children: Tags(vec![Tag::Text("hola".to_string())]),
                            ..Default::default()
                        })]),
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            })
        )
    }

    #[test]
    fn iterate_html() {
        let body = r#"<!DOCTYPE html><html><head><meta key="h" content="w" /></head><body><h1>hola</h1></body></html>"#;
        let already_parsed = Tag::HTMLTag(HTMLTag {
            name: "html".to_string(),
            children: Tags(vec![
                Tag::HTMLTag(HTMLTag {
                    name: "head".to_string(),
                    children: Tags(vec![Tag::HTMLTag(HTMLTag {
                        name: "meta".to_string(),
                        attrs: Attrs(vec![
                            Attr("key".to_string(), Some("h".to_string())),
                            Attr("content".to_string(), Some("w".to_string())),
                        ]),
                        ..Default::default()
                    })]),
                    ..Default::default()
                }),
                Tag::HTMLTag(HTMLTag {
                    name: "body".to_string(),
                    children: Tags(vec![Tag::HTMLTag(HTMLTag {
                        name: "h1".to_string(),
                        children: Tags(vec![Tag::Text("hola".to_string())]),
                        ..Default::default()
                    })]),
                    ..Default::default()
                }),
            ]),
            ..Default::default()
        });
        let mut already_parsed_iter = already_parsed.into_iter();

        let parsed = Tag::from_raw(body).unwrap();
        let mut parsed_iter = parsed.into_iter();

        while let (Some(already_parsed_tag), Some(parsed_tag)) =
            (already_parsed_iter.next(), parsed_iter.next())
        {
            assert_eq!(already_parsed_tag, parsed_tag);
        }
    }
}
