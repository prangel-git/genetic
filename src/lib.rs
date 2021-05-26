extern crate bitvec;
extern crate rand;

pub mod algorithm;
pub mod gen_real;
mod genetic;
mod chromosome;

pub use genetic::Genetic;
pub use chromosome::Chromosome;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
