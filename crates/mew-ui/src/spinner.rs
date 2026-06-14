use crate::phrases::phrase;

pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Debug, Clone)]
pub struct SpinnerFrame {
    pub frame: &'static str,
    pub text: &'static str,
}

pub fn spinner_frame(index: usize, category: &str) -> SpinnerFrame {
    SpinnerFrame {
        frame: SPINNER_FRAMES[index % SPINNER_FRAMES.len()],
        text: phrase(category),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_has_frames() {
        assert!(!SPINNER_FRAMES.is_empty());
    }
}
