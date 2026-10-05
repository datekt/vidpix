use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::core::charset::Charset;
use crate::i18n::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Mp4,
    Gif,
}

pub fn theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

pub fn choose_language() -> Result<Option<usize>> {
    let items = vec!["Русский", "English", "Cancel / Отмена"];
    let selection = Select::with_theme(&theme())
        .with_prompt("Language? / Язык?")
        .items(&items)
        .default(0)
        .interact_opt()?;
    match selection {
        Some(2) | None => Ok(None),
        Some(i) => Ok(Some(i)),
    }
}

pub fn choose_video(lang: &Lang, names: &[String]) -> Result<Option<usize>> {
    let mut items: Vec<String> = names.to_vec();
    items.push(lang.cancel.to_string());

    let selection = Select::with_theme(&theme())
        .with_prompt(lang.video_prompt)
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(i) if i == names.len() => Ok(None),
        Some(i) => Ok(Some(i)),
        None => Ok(None),
    }
}

pub fn choose_charset(lang: &Lang) -> Result<Option<Charset>> {
    let items = vec![
        lang.charset_binary,
        lang.charset_punct01,
        lang.charset_star,
        lang.charset_all,
        lang.cancel,
    ];

    let selection = Select::with_theme(&theme())
        .with_prompt(lang.charset_prompt)
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(0) => Ok(Some(Charset::Binary)),
        Some(1) => Ok(Some(Charset::Punct01)),
        Some(2) => Ok(Some(Charset::Star)),
        Some(3) => Ok(Some(Charset::All)),
        _ => Ok(None),
    }
}

pub fn choose_format(lang: &Lang) -> Result<Option<OutputFormat>> {
    let items = vec![lang.format_mp4, lang.format_gif, lang.cancel];

    let selection = Select::with_theme(&theme())
        .with_prompt(lang.format_prompt)
        .items(&items)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(0) => Ok(Some(OutputFormat::Mp4)),
        Some(1) => Ok(Some(OutputFormat::Gif)),
        _ => Ok(None),
    }
}