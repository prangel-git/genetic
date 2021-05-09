mod utils;

use utils::*;

use crate::Genetic;

use core::f64;

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub type Cache<T> = HashMap<Rc<T>, f64>;

/// Contains parameters for genetic algorithm
pub struct AlgorithmParams {
    pub rounds: usize,
    pub max_population: usize,
    pub mutation_rate: f64,
    pub co_rate: f64,
}

/// Runs a genetic algorithm starting from an initial population. It returns the fittest population.
pub fn genetic_algorithm<T>(
    initial_population: &Vec<Rc<T>>,
    params: &AlgorithmParams,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut Cache<T>,
) -> Vec<Rc<T>>
where
    T: Genetic + Hash + Eq,
{
    let mut population = initial_population_make(initial_population, params.max_population);

    for _ in 0..params.rounds {
        let dist = survival_probability_make(&population, fitness, cache);
        population = update_population(&population, &dist, params.mutation_rate, params.co_rate);
    }

    return population;
}
