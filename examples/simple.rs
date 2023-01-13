use clap::{arg, Command};
use klask::Settings;

fn main() {
    let app = Command::new("Example").arg(arg!(--debug <VALUE>).short('d'));
    klask::run_app(app, Settings::default(), |matches| {
        println!("{:?}", matches.try_contains_id("debug"))
    });
}
