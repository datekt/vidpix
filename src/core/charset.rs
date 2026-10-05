#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charset {
    Binary,
    Punct01,
    Star,
    All,
}

impl Charset {
    pub fn symbols(&self) -> &'static [char] {
        match self {
            Charset::Binary => &['0', '1'],
            Charset::Punct01 => &[':', ';', '0', '1'],
            Charset::Star => &[' ', '*'],
            Charset::All => &['0', '1', ':', ';', '*'],
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Charset::Binary => "01",
            Charset::Punct01 => ":;01",
            Charset::Star => "*",
            Charset::All => "all",
        }
    }
}