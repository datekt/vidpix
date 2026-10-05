use crate::i18n::Lang;

pub const LANG: Lang = Lang {
    code: "en",
    name: "English",

    cancel: "Cancel",

    video_prompt: "Hi! Which video do we convert?",
    no_videos: "No video files found next to the binary.",

    charset_prompt: "Great choice! Which characters for the animation?",
    charset_binary: "0 and 1",
    charset_punct01: ": ; 0 1",
    charset_star: "*",
    charset_all: "All together (0 1 : ; *)",

    format_prompt: "Which output format?",
    format_mp4: "MP4 (video)",
    format_gif: "GIF (animation)",

    probing: "Reading video info...",
    downloading_ffmpeg: "Downloading ffmpeg (first run only)...",
    converting: "Extracting frames....",
    rendering_frames: "Rendering ASCII frames...",
    encoding_video: "Encoding video...",

    done: "All done!",
    output_saved: "File saved: {}",
    cancelled: "Cancelled.",
    error_prefix: "Error",

    help_text: "\
VIDPIX — converts videos into ASCII animation (mp4/gif)

USAGE:
    vidpix              launch interactive menu
    vidpix --help       show this help
    vidpix --version    show version

HOW IT WORKS:
    1. Put the vidpix binary next to your video files
    2. Run vidpix
    3. Pick a video, character set and output format
    4. Get an mp4 or gif next to the source video

SUPPORTED VIDEO:
    mp4, mov, avi, mkv, webm, m4v, flv, wmv

NOTE:
    On the first run the tool will download ffmpeg next to the binary.
    After that everything works offline and portable.
",
};