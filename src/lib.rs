extern crate bitvec;

mod algorithm;
mod genetic;

pub use genetic::Chromosome;
pub use genetic::Genetic;

pub use algorithm::genetic_algorithm;
pub use algorithm::AlgorithmParams;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
