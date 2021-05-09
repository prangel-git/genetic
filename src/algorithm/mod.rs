mod utils;

use utils::*;

use crate::Genetic;

use core::f64;

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use rand::prelude::*;

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

    let mut rng_a = thread_rng();
    let mut rng_b = thread_rng();

    for _ in 0..params.rounds {
        let dist_a = surival_probability_make(&population, fitness, cache);
        let dist_b = dist_a.clone();

        population = dist_a
            .sample_iter(&mut rng_a)
            .zip(dist_b.sample_iter(&mut rng_b))
            .take(params.max_population)
            .map(|(i, j)| {
                reproduction(
                    &population[i],
                    &population[j],
                    params.mutation_rate,
                    params.co_rate,
                )
            })
            .collect::<Vec<_>>();
    }

    return population;
}
