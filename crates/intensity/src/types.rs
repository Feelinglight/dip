use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot {
    pub x: f64,
    pub y: f64,
}

impl Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}; {})", self.x, self.y)
    }
}
