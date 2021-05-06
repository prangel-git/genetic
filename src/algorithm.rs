use bitvec::prelude::*;
use super::Genetic;

/// Contains parameters for genetic algorithm
pub struct AlgorithmParams<'a, T> 
where 
T: Genetic
{
    rounds: u64,
    descendents_number: u64,
    max_popuation: u64,
    mutation_rate: f64, 
    co_rate: f64, 
    fitness: &'a dyn Fn(&T)->f64,
}

/// Runs a genetic algorithm starting from an initial population. It returns the fittest population.
pub fn genetic_algorithm<T>(
    initial_population: Vec<Box<T>>,
    params: &AlgorithmParams<T>,    
) -> Vec<Box<T>> 
where
T: Genetic {

    let mut population = if initial_population.is_empty() {
        vec![Box::new(T::from_chromosome(BitVec::new()))]
    } else {
        initial_population
    };
     
    todo!();
}