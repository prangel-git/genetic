use super::GenReal;
use crate::Genetic;

use std::hash::Hash;

impl Hash for GenReal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.choromosome().hash(state);
    }
}
