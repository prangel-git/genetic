use crate::Genetic;
use crate::GenotypeToFitness;

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
pub(super) fn fitness_proportion_distribution<T>(
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

/// Finds distribution based on number of wins in tournament
pub(super) fn tournament_wins<T>(
    population: &Vec<Rc<T>>,
    matching: &Box<dyn Fn(&T, &T) -> f64>,
) -> Vec<f64>
where
    T: Genetic + Hash + Eq,
{
    let mut wins = vec![0f64; population.len()];

    for idx_a in 0..population.len() {
        for idx_b in 0..population.len() {
            let result_match = matching(&population[idx_a], &population[idx_b]);
            wins[idx_a] += result_match;
            wins[idx_b] -= result_match;
        }
    }

    wins.iter().map(|x| x.max(0f64) + 1f64).collect()
}

/// Selects and reproduces a new population using a given distribution.
pub(super) fn roulette_wheel_selection<T>(
    population: &Vec<Rc<T>>,
    dist: &WeightedIndex<f64>,
    offspring_len: usize,
    mutation_rate: f64,
    co_rate: f64,
) -> Vec<Rc<T>>
where
    T: Genetic,
{
    let mut offspring = Vec::with_capacity(offspring_len);

    let mut rng = thread_rng();

    for _ in 0..offspring_len {
        let parent_a = &population[dist.sample(&mut rng)];
        let parent_b = &population[dist.sample(&mut rng)];

        let child = reproduction(parent_a, parent_b, mutation_rate, co_rate);

        offspring.push(child);
    }

    return offspring;
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
fn calc_fitness<T>(
    element: &Rc<T>,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut GenotypeToFitness<T>,
) -> f64
where
    T: Genetic + Hash + Eq,
{
    match cache.entry(element.clone()) {
        Entry::Vacant(entry) => *entry.insert(fitness(element)),
        Entry::Occupied(entry) => *entry.get(),
    }
}
