use clap::Parser;
use klask::Settings;

#[derive(Debug, Parser)]
struct Opts {
    #[arg(long)]
    opt1: Option<String>,
    #[arg(long)]
    opt2: Option<String>,
}

fn main() {
    klask::run_derived::<Opts, _>(Settings::default(), |opt| println!("{opt:?}"));
}
