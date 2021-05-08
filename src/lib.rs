extern crate bitvec;

mod algorithm;
mod gen_real;
mod genetic;

pub use genetic::Chromosome;
pub use genetic::Genetic;

pub use algorithm::genetic_algorithm;
pub use algorithm::AlgorithmParams;
pub use algorithm::Cache;

pub use gen_real::GenReal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
