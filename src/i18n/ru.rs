use crate::i18n::Lang;

pub const LANG: Lang = Lang {
    code: "ru",
    name: "Русский",

    cancel: "Отмена",

    video_prompt: "Привет! Какое видео конвентируем?",
    no_videos: "В папке с бинарником не найдено ни одного видеофайла.",

    charset_prompt: "Отличный выбор! Из каких символов хотите анимацию?",
    charset_binary: "0 и 1",
    charset_punct01: ": ; 0 1",
    charset_star: "*",
    charset_all: "Все вместе (0 1 : ; *)",

    format_prompt: "В каком формате сохранить?",
    format_mp4: "MP4 (видео)",
    format_gif: "GIF (анимация)",

    probing: "Читаю информацию о видео...",
    downloading_ffmpeg: "Скачиваю ffmpeg (только при первом запуске)...",
    converting: "Извлекаю кадры....",
    rendering_frames: "Рендерю ASCII-кадры...",
    encoding_video: "Собираю видео...",

    done: "Все готово!",
    output_saved: "Файл сохранён: {}",
    cancelled: "Отменено.",
    error_prefix: "Ошибка",

    help_text: "\
VIDPIX — превращает видео в ASCII-анимацию (mp4/gif)

ИСПОЛЬЗОВАНИЕ:
    vidpix              запуск интерактивного меню
    vidpix --help       показать эту справку
    vidpix --version    показать версию

КАК ЭТО РАБОТАЕТ:
    1. Положите бинарник vidpix рядом с видеофайлами
    2. Запустите vidpix
    3. Выберите видео, набор символов и формат вывода
    4. Получите mp4 или gif рядом с исходным видео

ПОДДЕРЖИВАЕМЫЕ ВИДЕО:
    mp4, mov, avi, mkv, webm, m4v, flv, wmv

ПРИМЕЧАНИЕ:
    При первом запуске утилита скачает ffmpeg рядом с бинарником.
    Дальше всё работает оффлайн и портативно.
",
};