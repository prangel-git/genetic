use super::chromosome::Chromosome;

/// Produces genetic material for genetic algorithm
pub trait Genetic {
    type Chromosome : Chromosome + Sized;

    /// Returns a random element
    fn new_random() -> Self;

    /// Returns the gene of an object
    fn choromosome(&self) -> Self::Chromosome;

    /// Returns object from chromosome
    fn from_chromosome(chromosome: Self::Chromosome) -> Self;

    /// Mutates the bits of a chromosome. Each bit is mutated with probability equals to mutation_rate.
    fn mutation(&self, mutation_rate: f64) -> Self
    where
        Self: Sized,
    {
        let chromosome = self.choromosome();

        let mutated = chromosome.mutation(mutation_rate);

        Genetic::from_chromosome(mutated)
    }

    /// Crosses the bits of two chromosomes. The crossover happens with probability co_rate.
    fn cross_over(&self, other: &Self, co_rate: f64) -> Self
    where
        Self: Sized,
    {
        let chromosome = self.choromosome();
        let chromosome_other = other.choromosome();

        let child = chromosome.cross_over(&chromosome_other, co_rate);

        Genetic::from_chromosome(child)
    }
}
