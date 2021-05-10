use genetic::*;

fn fitness(x_gen: &GenReal) -> f64 {
    let x = *x_gen.value();

    (x + 2f64) * (2f64 - x) * x * x + f64::EPSILON
}

fn matching(x: &GenReal, y: &GenReal) -> f64 {
    let fit_x = fitness(x);
    let fit_y = fitness(y);
    if fit_x > fit_y {
        1f64
    } else if fit_y > fit_x {
        -1f64
    } else {
        0f64
    }
}

fn main() {
    println!("min u {:}, fitness {:}", 0f64, fitness(&GenReal::new(0f64)));
    println!("max u {:}, fitness {:}", 2f64, fitness(&GenReal::new(2f64)));
    println!(
        "half u {:}, fitness {:}",
        2f64.sqrt(),
        fitness(&GenReal::new(2f64.sqrt()))
    );

    let f_test = GenReal::new(1f64);

    println!("Value f_test: {:?} ", f_test.value());
    println!("Chromosome f_test: {:?} ", f_test.choromosome());
    println!(
        "Recovered f_test: {:?} ",
        GenReal::from_chromosome(f_test.choromosome()).value()
    );

    let params = AlgorithmParams {
        rounds: 50,
        max_population: 10,
        mutation_rate: 0.1,
        co_rate: 0.5,
    };

    let mut cache = GenotypeToFitness::new();
    let fitness_b: Box<dyn Fn(&GenReal) -> f64> = Box::new(fitness);

    let last_population = ga_fitness_selection(&Vec::new(), &params, &fitness_b, &mut cache);

    let mut results = last_population
        .iter()
        .map(|x| (x.value(), fitness(x)))
        .collect::<Vec<_>>();

    results.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    println!("Last population:");

    for (val, fit) in results {
        println!(
            "Value {:.8}, Fitness Calculated {:.8}, Fitness stored {:.8}",
            val,
            fitness(&GenReal::new(*val)),
            fit
        );
    }

    println!("Full populaiton:");

    let mut cache_vec = cache.into_iter().collect::<Vec<(_, _)>>();
    cache_vec.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    for (val, fit) in cache_vec.iter().take(10) {
        println!(
            "Value {:.8}, Fitness Calculated {:.8}, Fitness stored {:.8}",
            val.value(),
            fitness(&val),
            fit
        );
    }

    let matching_b: Box<dyn Fn(&GenReal, &GenReal) -> f64> = Box::new(matching);

    let last_population = ga_tournament_selection(&Vec::new(), &params, &matching_b);

    println!("Last Tournament population:");

    for val in last_population {
        println!(
            "Value {:.8}, Fitness Calculated {:.8}",
            val.value(),
            fitness(&val)
        );
    }
}
