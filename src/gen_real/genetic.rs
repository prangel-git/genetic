
use super::GenReal;
use crate::genetic::Genetic;

use bitvec::prelude::*;
use rand::Rng;

impl Genetic for GenReal {
    type Chromosome = BitVec<Lsb0, u64>;

    fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let value = 4f64 * rng.gen::<f64>() - 2f64;

        GenReal::new(value)
    }

    fn choromosome(&self) -> Self::Chromosome {
        let value_bits = self.value.to_bits();
        Self::Chromosome::from_element(value_bits)
    }

    fn from_chromosome(chromosome: Self::Chromosome) -> Self {
        if chromosome.is_empty() {
            Genetic::new_random()
        } else {
            let mut chr = chromosome;
            chr.set(62, false); // Forces the exponent of f64 to be negative. Produces a number in (-2, 2)

            let value_u64 = chr.into_vec()[0];
            let value = f64::from_bits(value_u64);
            GenReal::new(value)
        }
    }
}
