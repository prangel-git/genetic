mod utils;

use rand::distributions::WeightedIndex;
use utils::*;

use crate::Genetic;

use core::f64;

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub type GenotypeToFitness<T> = HashMap<Rc<T>, f64>;

/// Contains parameters for genetic algorithm
pub struct AlgorithmParams {
    pub rounds: usize,
    pub max_population: usize,
    pub mutation_rate: f64,
    pub co_rate: f64,
}

/// Runs a genetic algorithm using fitness proportion selection.
pub fn ga_fitness_selection<T>(
    initial_population: &Vec<Rc<T>>,
    params: &AlgorithmParams,
    fitness: &Box<dyn Fn(&T) -> f64>,
    cache: &mut GenotypeToFitness<T>,
) -> Vec<Rc<T>>
where
    T: Genetic + Hash + Eq,
{
    let mut population = initial_population_make(initial_population, params.max_population);

    for _ in 0..params.rounds {
        let dist = fitness_proportion_distribution(&population, fitness, cache);

        population = roulette_wheel_selection(
            &population,
            &dist,
            population.len(),
            params.mutation_rate,
            params.co_rate,
        );
    }

    return population;
}

/// Runs a genetic algorith using tournament selection.
pub fn ga_tournament_selection<T>(
    initial_population: &Vec<Rc<T>>,
    params: &AlgorithmParams,
    matching: &Box<dyn Fn(&T, &T) -> f64>,
) -> Vec<Rc<T>>
where
    T: Genetic + Hash + Eq,
{
    let population_len = params.max_population;
    let mut population = initial_population_make(initial_population, population_len);

    let offspring_len = population_len - 1;
    let mut offspring;

    for _ in 0..params.rounds {
        let wins = tournament_wins(&population, matching);

        let dist = WeightedIndex::new(&wins).unwrap();

        offspring = roulette_wheel_selection(
            &population,
            &dist,
            offspring_len,
            params.mutation_rate,
            params.co_rate,
        );

        let mut idxs = (0..population_len).collect::<Vec<usize>>();

        idxs.sort_by(|x, y| wins[*y].partial_cmp(&wins[*x]).unwrap());

        population = idxs
            .iter()
            .map(|&x| population[x].clone())
            .take(population_len - offspring_len)
            .collect::<Vec<_>>();

        population.append(&mut offspring);
    }

    return population;
}
