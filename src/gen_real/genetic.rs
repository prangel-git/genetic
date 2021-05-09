use crate::Chromosome;
use crate::GenReal;
use crate::Genetic;

use rand::Rng;

impl Genetic for GenReal {
    fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let value = 4f64 * rng.gen::<f64>() - 2f64;

        GenReal::new(value)
    }

    fn choromosome(&self) -> Chromosome {
        let value_bits = self.value.to_bits();
        Chromosome::from_element(value_bits)
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
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
