use super::GenReal;
use crate::genetic::Genetic;

impl PartialEq for GenReal {
    fn eq(&self, other: &Self) -> bool {
        self.choromosome() == other.choromosome()
    }
}

impl Eq for GenReal {}
