use crate::Cache;
use crate::Genetic;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use rand::distributions::WeightedIndex;

use bitvec::prelude::*;

/// If the initial population is empty, it spontanously generates an individual.
pub(super) fn initial_population_make<T>(initial_population: &Vec<Rc<T>>) -> Vec<Rc<T>>
where
    T: Genetic,
{
    if initial_population.is_empty() {
        vec![Rc::new(T::from_chromosome(BitVec::new()))]
    } else {
        initial_population.clone()
    }
}

/// Finds the distribution for survival of a population based on a fitness function.
pub(super) fn surival_probability_make<T>(
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
pub(super) fn reproduction<T>(
    parent_a: &Rc<T>,
    parent_b: &Rc<T>,
    mutation_rate: f64,
    co_rate: f64,
) -> Rc<T>
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
