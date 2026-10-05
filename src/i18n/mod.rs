mod en;
mod ru;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Ru,
    En,
}

#[derive(Clone, Copy)]
pub struct Lang {
    #[allow(dead_code)]
    pub code: &'static str,
    #[allow(dead_code)]
    pub name: &'static str,

    pub cancel: &'static str,

    pub video_prompt: &'static str,
    pub no_videos: &'static str,

    pub charset_prompt: &'static str,
    pub charset_binary: &'static str,
    pub charset_punct01: &'static str,
    pub charset_star: &'static str,
    pub charset_all: &'static str,

    pub format_prompt: &'static str,
    pub format_mp4: &'static str,
    pub format_gif: &'static str,

    pub probing: &'static str,
    pub downloading_ffmpeg: &'static str,
    pub converting: &'static str,
    pub rendering_frames: &'static str,
    pub encoding_video: &'static str,

    pub done: &'static str,
    pub output_saved: &'static str,
    pub cancelled: &'static str,
    pub error_prefix: &'static str,

    pub help_text: &'static str,
}

pub fn get(language: Language) -> Lang {
    match language {
        Language::Ru => ru::LANG,
        Language::En => en::LANG,
    }
}