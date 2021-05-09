mod eq;
mod genetic;
mod hash;

// use super::*;

/// Implements a real number with the genetic trait.
pub struct GenReal {
    value: f64,
}

impl GenReal {
    pub fn new(value: f64) -> Self {
        GenReal { value }
    }

    pub fn value(&self) -> &f64 {
        &self.value
    }
}
