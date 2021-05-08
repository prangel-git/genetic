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
        let value_bytes = self.value.to_be_bytes();
        let value_usize = usize::from_be_bytes(value_bytes);
        Chromosome::from_element(value_usize)
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
        if chromosome.is_empty() {
            GenReal { value: 0f64 }
        } else {
            let mut chr = chromosome;
            chr.set(62, false); // Forces sign of f64 to be positive
            chr.set(63, false); // Forces exponent of f64 to be negative

            let value_usize = chr.into_vec()[0];
            let value_bytes = value_usize.to_be_bytes();
            let value = f64::from_be_bytes(value_bytes);
            GenReal { value }
        }
    }
}
