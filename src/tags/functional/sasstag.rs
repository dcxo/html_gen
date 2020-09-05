use anyhow::Result;
use std::{fmt::Display, io::Write, path::PathBuf};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SassTag {
    dst_file_path: PathBuf,
}

impl SassTag {
    pub fn new(src: &str) -> Result<Self> {
        let sass_file = PathBuf::from(src);

        let d = rsass::compile_scss_file(&sass_file, rsass::output::Format::default())?;

        let mut dst_file_path = PathBuf::from("dist");
        dst_file_path.push("style");
        std::fs::create_dir_all(&dst_file_path)?;

        dst_file_path.push(sass_file.file_name().unwrap());
        dst_file_path.set_extension("css");

        let mut dst_file = std::fs::File::create(&dst_file_path)?;
        dst_file.write_all(&d)?;

        Ok(SassTag { dst_file_path })
    }
}

impl Display for SassTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<link rel="stylesheet" href="{}" />"#,
            self.dst_file_path.strip_prefix("dist").unwrap().display()
        )
    }
}
