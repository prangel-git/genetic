extern crate bitvec;

mod genetic;
mod algorithm;

pub use genetic::Genetic;

pub use algorithm::AlgorithmParams;
pub use algorithm::genetic_algorithm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
