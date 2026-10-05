use vidpix::core::charset::Charset;
use vidpix::core::converter::{brightness_to_char, frame_to_ascii};

#[test]
fn binary_charset_black_is_zero() {
    assert_eq!(brightness_to_char(0, Charset::Binary), '0');
}

#[test]
fn binary_charset_white_is_one() {
    assert_eq!(brightness_to_char(255, Charset::Binary), '1');
}

#[test]
fn binary_charset_mid_is_one() {
    assert_eq!(brightness_to_char(128, Charset::Binary), '1');
}

#[test]
fn punct01_charset_range() {
    let symbols = Charset::Punct01.symbols();
    assert_eq!(symbols.len(), 4);

    assert_eq!(brightness_to_char(0, Charset::Punct01), ':');
    assert_eq!(brightness_to_char(255, Charset::Punct01), '1');
}

#[test]
fn star_charset_low_is_space() {
    assert_eq!(brightness_to_char(0, Charset::Star), ' ');
}

#[test]
fn star_charset_high_is_star() {
    assert_eq!(brightness_to_char(255, Charset::Star), '*');
}

#[test]
fn all_charset_returns_valid_symbols() {
    let valid = Charset::All.symbols();
    for px in 0..=255u8 {
        let c = brightness_to_char(px, Charset::All);
        assert!(valid.contains(&c), "unexpected char {:?} for px {}", c, px);
    }
}

#[test]
fn frame_to_ascii_dimensions() {
    let width = 4u32;
    let height = 2u32;
    let frame = vec![0u8; (width * height) as usize];

    let ascii = frame_to_ascii(&frame, width, height, Charset::Binary);

    let lines: Vec<&str> = ascii.lines().collect();
    assert_eq!(lines.len(), height as usize);
    for line in &lines {
        assert_eq!(line.chars().count(), width as usize);
    }
}

#[test]
fn frame_to_ascii_all_zeros() {
    let frame = vec![0u8; 6];
    let ascii = frame_to_ascii(&frame, 3, 2, Charset::Binary);
    assert_eq!(ascii, "000\n000\n");
}

#[test]
fn frame_to_ascii_all_whites() {
    let frame = vec![255u8; 6];
    let ascii = frame_to_ascii(&frame, 3, 2, Charset::Binary);
    assert_eq!(ascii, "111\n111\n");
}

#[test]
fn frame_to_ascii_mixed() {
    let frame = vec![0u8, 255, 0, 255];
    let ascii = frame_to_ascii(&frame, 2, 2, Charset::Binary);
    assert_eq!(ascii, "01\n01\n");
}

#[test]
fn charset_labels_are_stable() {
    assert_eq!(Charset::Binary.label(), "01");
    assert_eq!(Charset::Punct01.label(), ":;01");
    assert_eq!(Charset::Star.label(), "*");
    assert_eq!(Charset::All.label(), "all");
}