use bitvec::prelude::*;
use rand::Rng;

pub type Chromosome = BitVec;

/// Produces genetic material for genetic algorithm
pub trait Genetic {
    /// Returns the gene of an object
    fn choromosome(&self) -> Chromosome;

    /// Returns object from chromosome
    fn from_chromosome(chromosome: Chromosome) -> Self;

    /// Mutates the bits of a chromosome. Each bit is mutated with probability equals to mutation_rate.
    fn mutation(&self, mutation_rate: f64) -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();

        let mut chromosome = self.choromosome();

        for gene in &mut chromosome {
            let rnd = rng.gen::<f64>();
            if rnd < mutation_rate {
                let new_value = !(*gene);
                gene.set(new_value);
            }
        }

        Genetic::from_chromosome(chromosome)
    }

    /// Crosses the bits of two chromosomes. The crossover happens with probability co_rate.
    fn cross_over(&self, other: &Self, co_rate: f64) -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();

        let mut chromosome = self.choromosome();
        let chromosome_other = other.choromosome();

        for (gene, gene_other) in chromosome.iter_mut().zip(chromosome_other) {
            let rnd = rng.gen::<f64>();
            if rnd < co_rate {
                gene.set(gene_other);
            }
        }

        Genetic::from_chromosome(chromosome)
    }
}
