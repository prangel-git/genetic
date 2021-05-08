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
        let dist_a = surival_probability_make(&population, fitness, cache);
        let dist_b = surival_probability_make(&population, fitness, cache);

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

/// Finds the distribution for survival of a population based on a fitness function.
fn surival_probability_make<T>(
    population: &Vec<Rc<T>>,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut HashMap<Rc<T>, f64>,
) -> WeightedIndex<f64>
where
    T: Genetic + Hash + Eq,
{
    WeightedIndex::new(
        population
            .iter()
            .map(|sample| calc_fitness(sample, fitness, cache)),
    )
    .unwrap()
}

/// Produces offspring from two parents.
fn reproduction<T>(parent_a: &Rc<T>, parent_b: &Rc<T>, mutation_rate: f64, co_rate: f64) -> Rc<T>
where
    T: Genetic,
{
    Rc::new(
        parent_a
            .cross_over(parent_b, co_rate)
            .mutation(mutation_rate),
    )
}
