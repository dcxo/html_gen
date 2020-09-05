use anyhow::{anyhow, Context, Result};
use json::JsonValue;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// data_path_crawler uses the vec, tht comes from a string that is splitted
/// to crawl the json to get a final value
pub fn data_path_crawler(in_json: &JsonValue, vec: &Vec<&str>) -> Result<JsonValue> {
    let mut to_return = in_json;
    for next in vec {
        match to_return {
            JsonValue::Array(avec) => {
                let idx = next
                    .parse::<usize>()
                    .context(format!("Array index expected, found: {}", next))?;

                to_return = &avec[idx];
            }
            JsonValue::Object(obj) => {
                to_return = obj.get(next).context("This key doesn't exist")?;
            }
            _ => {
                return Err(anyhow!("Do you know how to write json files at all?"));
            }
        }
    }
    Ok(to_return.clone())
}

pub fn get_data(path: &str) -> Result<JsonValue> {
    let mut vec = path.split('.').map(str::trim);
    let stem = vec.next().unwrap();

    let mut file = File::open(PathBuf::from("data").join(format!("{}.json", stem)))
        .context(format!("File {}.json not found", stem))?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    data_path_crawler(&json::parse(&file_content)?, &vec.collect())
}

/// expand_data_fragment expands data that comes from a single source
pub fn expand_data_fragment(content: &mut String, value: &JsonValue) -> Result<()> {
    while let Some(idx_start) = content.find("{{") {
        let idx_end = content.find("}}").context("Error with data binding")? + 2;
        let path = &content[(idx_start + 2)..(idx_end - 2)]
            .split('.')
            .map(str::trim)
            .collect::<Vec<_>>();

        let to_return = data_path_crawler(value, path)?;

        content.replace_range(idx_start..idx_end, &format!("{}", to_return));
    }
    Ok(())
}

/// expand_data expands data that can comes from different json files
pub fn expand_data(content: &mut String) -> Result<()> {
    while let Some(s) = content.find("{{") {
        let e = content.find("}}").context("Error with data binding")? + 2;

        let data = get_data(&content[(s + 2)..(e - 2)])?;
        content.replace_range(s..e, &format!("{}", data));
    }
    Ok(())
}
