use anyhow::{anyhow, Context, Result};
use json;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// path is in the format summ.summ.summ.summ...
pub fn get_data(path: &str) -> Result<json::JsonValue> {
    let mut vec = path.split(".").into_iter();
    let stem = vec.next().unwrap();

    let mut file = File::open(PathBuf::from("data").join(format!("{}.json", stem)))?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let mut to_return = json::parse(&file_content)?;
    for next in vec.map(str::trim) {
        match to_return {
            json::JsonValue::Array(avec) => {
                let idx = next
                    .parse::<usize>()
                    .context(format!("Array index expected, found: {}", next))?;

                to_return = (&avec[idx]).clone();
            }
            json::JsonValue::Object(obj) => {
                to_return = obj.get(next).context("This key doesn't exist")?.clone();
            }
            _ => {
                return Err(anyhow!("Do you know how to write json files at all?"));
            }
        }
    }
    Ok(to_return)
}
