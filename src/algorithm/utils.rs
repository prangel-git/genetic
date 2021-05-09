use crate::Cache;
use crate::Genetic;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// If the initial population is smaller than the max_population, it completes it with random elements
pub(super) fn initial_population_make<T>(
    initial_population: &Vec<Rc<T>>,
    max_population: usize,
) -> Vec<Rc<T>>
where
    T: Genetic,
{
    let mut population = initial_population.clone();

    while population.len() < max_population {
        population.push(Rc::new(T::new_random()))
    }

    return population;
}

/// Finds the distribution for survival of a population based on a fitness function.
pub(super) fn survival_probability_make<T>(
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

/// Updates the current population
pub(super) fn update_population<T>(
    population: &Vec<Rc<T>>,
    dist: &WeightedIndex<f64>,
    mutation_rate: f64,
    co_rate: f64,
) -> Vec<Rc<T>>
where
    T: Genetic,
{
    let pop_len = population.len();
    let mut next_population = Vec::with_capacity(pop_len);

    let mut rng = thread_rng();

    for _ in 0..pop_len {
        let parent_a = &population[dist.sample(&mut rng)];
        let parent_b = &population[dist.sample(&mut rng)];

        let child = reproduction(parent_a, parent_b, mutation_rate, co_rate);

        next_population.push(child);
    }

    return next_population;
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
