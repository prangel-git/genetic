extern crate bitvec;

mod algorithm;
mod gen_real;
mod genetic;

pub use genetic::Chromosome;
pub use genetic::Genetic;

pub use algorithm::ga_fitness_selection;
pub use algorithm::ga_tournament_selection;
pub use algorithm::AlgorithmParams;
pub use algorithm::GenotypeToFitness;

pub use gen_real::GenReal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
