pub mod bitvec_chr;

pub trait Chromosome {
    fn mutation(&self, mutation_rate: f64) -> Self;

    fn cross_over(&self, other: &Self, co_rate: f64) -> Self;
}