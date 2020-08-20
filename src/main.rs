use anyhow::{ensure, Context, Result};
use clap::Clap;
use colored::Colorize;
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{create_dir, File},
    io::{self, Read, Write},
    path::Path,
    process::exit,
};

mod command;
mod component;
mod data;
mod expander;

use command::*;

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
    if proj_name.is_none() {
        println!("How is your project named? ");

        project_name = String::new();
        if let Err(e) = io::stdin().read_line(&mut project_name) {
            eprintln!("There was an error: {}", e);
        };
    } else {
        project_name = proj_name.unwrap(); // unwrap safe
    }
    let path = std::path::PathBuf::from(project_name.trim());

    create_dir(&path)?;
    create_dir(&path.join("components"))?;
    create_dir(&path.join("data"))?;
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

    let mut components_map = HashMap::<OsString, component::Component>::new();

    std::fs::read_dir("components")?.for_each(|entry| {
        let file_path = entry.unwrap().path();
        let stem = file_path.file_stem().unwrap();

        let comp = component::Component::from_file(&mut File::open(&file_path).unwrap()).unwrap();
        components_map.insert(stem.to_os_string(), comp);
    });

    println!("{}. Reading index.html file", "Building".cyan().bold());

    let mut index_file = File::open("index.html").context("Could not read index.html")?;
    let mut buf = vec![];
    index_file
        .read_to_end(&mut buf)
        .context("Could not read index.html.")?;
    let mut index_content = String::from_utf8(buf)?;
    index_content = expander::expand_components(components_map, &mut index_content);
    println!("{}. Reading components", "Expanding".cyan().bold());
    println!("{}. Reading data", "Binding".cyan().bold());

    index_content = expander::expand_data(&mut index_content);

    let dst_dir = Path::new("dist");
    if !dst_dir.exists() {
        create_dir(dst_dir).context("Unable to create 'dist' dir")?;
    }
    let mut index_file = File::create(dst_dir.join("index.html"))?;
    index_file.write_all(index_content.as_bytes())?;

    println!("{}", " Builded! ".on_cyan().black().bold());

    Ok(())
}
