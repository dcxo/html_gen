use anyhow::{anyhow, Context, Result};
use json::JsonValue;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn get_data(path: &str) -> Result<JsonValue> {
    let mut vec = path.split('.').map(str::trim);
    let stem = vec.next().unwrap();

    let mut file = File::open(PathBuf::from("data").join(format!("{}.json", stem)))?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let mut to_return = json::parse(&file_content)?;
    for next in vec {
        match to_return {
            JsonValue::Array(avec) => {
                let idx = next
                    .parse::<usize>()
                    .context(format!("Array index expected, found: {}", next))?;

                to_return = (&avec[idx]).clone();
            }
            JsonValue::Object(obj) => {
                to_return = obj.get(next).context("This key doesn't exist")?.clone();
            }
            _ => {
                return Err(anyhow!("Do you know how to write json files at all?"));
            }
        }
    }
    Ok(to_return)
}

pub fn expand_data(content: &mut String) -> Result<()> {
    while let Some(s) = content.find("{{") {
        let e = content.find("}}").context("Error with data binding")? + 2;

        content.replace_range(s..e, &format!("{}", get_data(&content[(s + 2)..(e - 2)])?));
    }
    Ok(())
}
