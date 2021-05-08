use std::rc::Rc;

use bitvec::prelude::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::Genetic;

/// Contains parameters for genetic algorithm
pub struct AlgorithmParams
{
    pub rounds: usize,
    pub max_population: usize,
    pub mutation_rate: f64,
    pub co_rate: f64,
}

/// Runs a genetic algorithm starting from an initial population. It returns the fittest population.
pub fn genetic_algorithm<T>(
    initial_population: &Vec<Rc<T>>,
    params: &AlgorithmParams,
    fitness: Box<dyn Fn(&T) -> usize>,
) -> Vec<Rc<T>>
where
    T: Genetic,
{
    let mut population = if initial_population.is_empty() {
        vec![Rc::new(T::from_chromosome(BitVec::new()))]
    } else {
        initial_population.clone()
    };

    let mut rng = thread_rng();

    for _ in 0..params.rounds {
        let population_fitness = population
            .iter()
            .map(|sample| fitness(sample))
            .collect::<Vec<_>>();

        let dist = WeightedIndex::new(population_fitness).unwrap();

        let mut population_next = Vec::new();

        while population_next.len() < params.max_population {
            let parent_a = &population[dist.sample(&mut rng)];
            let parent_b = &population[dist.sample(&mut rng)];

            let child = Rc::new(
                parent_a
                    .cross_over(parent_b, params.co_rate)
                    .mutation(params.mutation_rate),
            );

            population_next.push(child);
        }

        population = population_next;
    }

    return population;
}
