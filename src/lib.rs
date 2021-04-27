extern crate bit_vec;

mod genetic;
mod algorithm;

pub use genetic::Genetic;

pub use algorithm::algorithm_params;
pub use algorithm::genetic_algorithm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
