use bit_vec::BitVec;

type Chromosome = BitVec;

/// Produces genetic material for genetic algorithm
pub trait Genetic {

    /// Returns the gene of an object
    fn gene(&self) -> Chromosome;

    /// Returns object from gene
    fn from_gene(chromosome: Chromosome) -> Self;

    /// Mutation
    fn mutation(&self, mutation_rate : f64) -> Self where Self: Sized {
        todo!();
    }

    /// Crossover
    fn cross_over(&self, other: &Self, co_factor: f64) -> (Self, Self) where Self: Sized {
        todo!();
    }
}