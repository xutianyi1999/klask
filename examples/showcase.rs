//! Showcases clap parsing and different widgets
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::{Parser, ValueHint};
use klask::Settings;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "App name")]
/// Help is displayed at the top
pub struct Showcase {
    /// Argument help is displayed as tooltips
    required_field: String,
    #[arg(long)]
    optional_field: Option<String>,
    #[arg(long, default_value = "default value")]
    field_with_default: String,
    #[arg(long)]
    flag: bool,
    #[arg(short, long, action = clap::ArgAction::Count)]
    count_occurrences_as_a_nice_counter: u8,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Subcommands also display help
    SubcommandA {
        #[arg(long, value_hint = ValueHint::AnyPath)]
        native_path_picker: Option<PathBuf>,
        #[arg(value_parser = ["One", "Two", "Three"])]
        choose_one: String,
        #[clap(subcommand)]
        inner: InnerSubcommand,
    },
    SubcommandB {},
}

#[derive(Debug, Parser)]
pub enum InnerSubcommand {
    InnerSubcommandA {
        #[arg(short, long)]
        multiple_values: Vec<String>,
    },
    /// About
    InnerSubcommandB {
        #[clap(subcommand)]
        inner: InnerInnerSubcommand,
    },
    InnerSubcommandC,
    InnerSubcommandD,
}

#[derive(Debug, Parser)]
pub enum InnerInnerSubcommand {
    /// About 2
    A,
    B,
}

fn main() {
    klask::run_derived::<Showcase, _>(Settings::default(), |o| println!("{o:#?}"));
}
