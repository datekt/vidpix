mod args;
mod cli;
mod i18n;
mod ui;
mod utils;

use vidpix::core;

use args::Action;
use i18n::Language;

fn main() {
    let result = match args::parse() {
        Action::ShowVersion => {
            args::print_version();
            return;
        }
        Action::ShowHelp(lang) => {
            args::print_help(lang);
            return;
        }
        Action::Run => cli::run(),
    };

    if let Err(e) = result {
        let lang = i18n::get(Language::Ru);
        eprintln!("{}: {}", lang.error_prefix, e);
        std::process::exit(1);
    }
}