use indicatif::{ProgressBar, ProgressStyle};

pub struct ConverterProgress {
    bar: ProgressBar,
}

impl ConverterProgress {
    pub fn new(label: &str) -> Self {
        let bar = ProgressBar::new(100);
        let style = ProgressStyle::with_template(
            "{msg}\n{bar:30} {percent}%",
        )
        .unwrap()
        .progress_chars("-#");
        bar.set_style(style);
        bar.set_message(label.to_string());
        Self { bar }
    }

    pub fn update(&self, done: u32, total: u32) {
        if total == 0 {
            return;
        }
        let pct = ((done as f64 / total as f64) * 100.0).min(100.0) as u64;
        self.bar.set_position(pct);
    }

    pub fn finish(&self) {
        self.bar.set_position(100);
        self.bar.finish_and_clear();
    }
}