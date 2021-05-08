use rand::Rng;

use std::hash::Hash;

use super::*;

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

impl Genetic for GenReal {
    fn choromosome(&self) -> Chromosome {
        let value_bits = self.value.to_bits();
        Chromosome::from_element(value_bits)
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
        if chromosome.is_empty() {
            let mut rng = rand::thread_rng();
            let value = 4f64 * rng.gen::<f64>() - 2f64; 
            
            GenReal::new(value)
        } else {
            let mut chr = chromosome;
            chr.set(62, false);  // Forces the exponent of f64 to be negative. Produces a number in (-2, 2)

            let value_u64 = chr.into_vec()[0];
            let value = f64::from_bits(value_u64);
            GenReal::new(value)
        }
    }
}

impl Hash for GenReal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.choromosome().hash(state);
    }
}

impl PartialEq for GenReal {
    fn eq(&self, other: &Self) -> bool {
        self.choromosome() == other.choromosome()
    }
}

impl Eq for GenReal {}