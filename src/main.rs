use anyhow::{ensure, Context, Result};
use clap::Clap;
use colored::Colorize;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

mod command;
mod component;
mod data;
mod tags;

use command::*;
use tags::Tag;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    if let Err(e) = match opts.subcmd {
        SubCMD::Create(CreateSubCMD { proj_name: pn }) => create_proj(pn),
        SubCMD::Build(_) => build_proj(),
    } {
        eprintln!("{}: {}", "error".bold().red(), e);
        exit(1)
    };
    Ok(())
}

/// create_proj is the function in charge of creating
/// the basic structure of a html_gen project
fn create_proj(proj_name: Option<String>) -> Result<()> {
    let mut project_name: String;
    match proj_name {
        Some(proj_name) => project_name = proj_name,
        None => {
            print!("How is your project named? ");
            let _ = io::stdout().flush();

            project_name = String::new();
            io::stdin().read_line(&mut project_name)?;
            project_name.retain(|c| !c.is_whitespace());
        }
    }
    let path = PathBuf::from(&project_name);

    fs::create_dir(&path)?;
    fs::create_dir(&path.join("components"))?;
    fs::create_dir(&path.join("data"))?;
    File::create(&path.join("index.html"))?;

    println!(
        "{} Your project has been created successfully",
        "Okay!".green().bold()
    );

    println!(
        "To start working on it, do: {}",
        format!("cd {}", project_name).underline()
    );

    Ok(())
}

/// build_proj is in charge of building the projects
fn build_proj() -> Result<()> {
    ensure!(
        Path::new("index.html").exists() && Path::new("components").exists(),
        "this folder does not match with a html_gen project",
    );

    println!("{}. Reading components", "Reading".cyan().bold());

    println!("{}. Reading index.html file", "Building".cyan().bold());

    let mut index_file = File::open("index.html").context("Could not read index.html")?;
    let mut buf = vec![];
    index_file
        .read_to_end(&mut buf)
        .context("Could not read index.html.")?;
    let mut index_content = String::from_utf8(buf)?;

    let index_parsed = Tag::from_raw(&index_content).context("Could not parse index.html")?;

    println!("{}. Expanding macros", "Expanding".cyan().bold());

    index_content = format!("{}", index_parsed);

    println!("{}. Reading data", "Binding".cyan().bold());

    data::expand_data(&mut index_content)?;

    #[cfg(debug_assertions)]
    println!("{}", index_content);

    let dst_dir = Path::new("dist");
    if !dst_dir.exists() {
        fs::create_dir(dst_dir).context("Unable to create 'dist' dir")?;
    }
    let mut index_file = File::create(dst_dir.join("index.html"))?;
    index_file.write_all(index_content.as_bytes())?;

    println!("{}", " Builded! ".on_cyan().black().bold());

    Ok(())
}
