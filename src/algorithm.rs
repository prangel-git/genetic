use core::f64;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use bitvec::prelude::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::Genetic;

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
    let mut population = initial_population_make(initial_population);

    let mut rng_a = thread_rng();
    let mut rng_b = thread_rng();

    for _ in 0..params.rounds {
        let dist_a = WeightedIndex::new(
            population
                .iter()
                .map(|sample| calc_fitness(sample, fitness, cache)),
        )
        .unwrap();

        let dist_b = WeightedIndex::new(
            population
                .iter()
                .map(|sample| calc_fitness(sample, fitness, cache)),
        )
        .unwrap();

        population = dist_a
            .sample_iter(&mut rng_a)
            .zip(dist_b.sample_iter(&mut rng_b))
            .take(params.max_population)
            .map(|(i, j)| {
                Rc::new(
                    population[i]
                        .cross_over(&population[j], params.co_rate)
                        .mutation(params.mutation_rate),
                )
            })
            .collect::<Vec<_>>();

        // population = population_n;
    }

    return population;
}

/// If the initial population is empty, it spontanously generates an individual.
fn initial_population_make<T>(initial_population: &Vec<Rc<T>>) -> Vec<Rc<T>>
where
    T: Genetic,
{
    if initial_population.is_empty() {
        vec![Rc::new(T::from_chromosome(BitVec::new()))]
    } else {
        initial_population.clone()
    }
}

/// Calcuates fitness and updates cache
fn calc_fitness<T>(element: &Rc<T>, fitness: &Box<dyn Fn(&T) -> f64>, cache: &mut Cache<T>) -> f64
where
    T: Genetic + Hash + Eq,
{
    match cache.entry(element.clone()) {
        Entry::Vacant(entry) => *entry.insert(fitness(element)),
        Entry::Occupied(entry) => *entry.get(),
    }
}
