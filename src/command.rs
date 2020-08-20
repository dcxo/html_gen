use clap::Clap;
use std::fmt::Debug;

#[derive(Debug, Clap)]
#[clap(version = "0.0.1", author = "David C. <im@dcxo.dev>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCMD,
}

#[derive(Debug, Clap)]
pub enum SubCMD {
    #[clap(alias = "c")]
    Create(CreateSubCMD),
    #[clap(alias = "b")]
    Build(BuildSubCMD),
}

#[derive(Debug, Clap)]
pub struct CreateSubCMD {
    pub proj_name: Option<String>,
}

#[derive(Debug, Clap)]
pub struct BuildSubCMD;
