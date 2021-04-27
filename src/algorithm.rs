use super::Genetic;

pub struct algorithm_params<'a, T> 
where 
T: Genetic
{
    rounds: u64,
    descendents_number: u64,
    max_popuation: u64,
    mutation_rate: f64, 
    co_factor: f64, 
    fitness: &'a dyn Fn(&T)->f64,
}

pub fn genetic_algorithm<T>(
    initial_population: Vec<T>,
    params: &algorithm_params<T>,    
) -> Vec<T> 
where
T: Genetic {
    todo!();
}