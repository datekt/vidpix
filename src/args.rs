use crate::i18n;
use crate::i18n::Language;

pub enum Action {
    Run,
    ShowVersion,
    ShowHelp(Language),
}

pub fn parse() -> Action {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if let Some(first) = args.first() {
        match first.as_str() {
            "--version" | "-V" | "-v" => return Action::ShowVersion,
            "--help" | "-h" => return Action::ShowHelp(detect_lang_from_args(&args)),
            _ => {}
        }
    }

    Action::Run
}

fn detect_lang_from_args(args: &[String]) -> Language {
    for a in args {
        match a.as_str() {
            "--lang=ru" | "--ru" => return Language::Ru,
            "--lang=en" | "--en" => return Language::En,
            _ => {}
        }
    }
    Language::Ru
}

pub fn print_version() {
    println!("v{}", env!("CARGO_PKG_VERSION"));
}

pub fn print_help(lang: Language) {
    let l = i18n::get(lang);
    println!("{}", l.help_text);
    println!("build: {} / {}", env!("VIDPIX_GIT_HASH"), env!("VIDPIX_TARGET"));
}