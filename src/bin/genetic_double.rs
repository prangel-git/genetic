use genetic::*;

fn fitness(x: &GenReal) -> usize {
    (10000f64 * x.value() * (2f64 - x.value()) + 1f64).max(1f64) as usize
}

fn main() {
    println!("min u {:}, fitness {:}", 0f64, fitness(&GenReal::new(0f64)));
    println!("max u {:}, fitness {:}", 2f64, fitness(&GenReal::new(2f64)));
    println!(
        "half u {:}, fitness {:}",
        1f64,
        fitness(&GenReal::new(1f64))
    );

    let f_test = GenReal::new(1f64);

    println!("Value f_test: {:?} ", f_test.value());
    println!("Chromosome f_test: {:?} ", f_test.choromosome());
    println!(
        "Recovered f_test: {:?} ",
        GenReal::from_chromosome(f_test.choromosome()).value()
    );

    let params = AlgorithmParams {
        rounds: 100,
        max_population: 10,
        mutation_rate: 0.05,
        co_rate: 0.01,
    };

    let last_population = genetic_algorithm(&Vec::new(), &params, Box::new(fitness));

    let mut results = last_population
        .iter()
        .map(|x| (x.value(), fitness(x)))
        .collect::<Vec<_>>();

    results.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (val, fit) in results {
        println!("Value {:?}, Fitness {:?}", val, fit);
    }
}
