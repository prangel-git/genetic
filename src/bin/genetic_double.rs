use genetic::*;

struct DoubleGen {
    value: f64,
}

impl Genetic for DoubleGen {
    fn choromosome(&self) -> Chromosome {
        let value_bytes = self.value.to_be_bytes();
        let value_usize = usize::from_be_bytes(value_bytes);
        Chromosome::from_element(value_usize)
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
        if chromosome.is_empty() {
            DoubleGen { value: 0f64 }
        } else {
            let mut chr = chromosome;
            chr.set(62, false); // Forces sign of f64 to be positive
            chr.set(63, false); // Forces exponent of f64 to be negative

            let value_usize = chr.into_vec()[0];
            let value_bytes = value_usize.to_be_bytes();
            let value = f64::from_be_bytes(value_bytes);
            DoubleGen { value }
        }
    }
}

fn fitness(x: &DoubleGen) -> usize {
    (10000f64 * x.value * (2f64 - x.value) + 1f64).max(1f64) as usize
}

fn main() {
    println!(
        "min u {:}, fitness {:}",
        0f64,
        fitness(&DoubleGen { value: 0f64 })
    );
    println!(
        "max u {:}, fitness {:}",
        2f64,
        fitness(&DoubleGen { value: 2f64 })
    );
    println!(
        "half u {:}, fitness {:}",
        1f64,
        fitness(&DoubleGen { value: 1f64 })
    );

    let f_test = DoubleGen { value: 2f64 };

    println!("Value f_test: {:?} ", f_test.value);
    println!("Chromosome f_test: {:?} ", f_test.choromosome());
    println!(
        "Recovered f_test: {:?} ",
        DoubleGen::from_chromosome(f_test.choromosome()).value
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
        .map(|x| (x.value, fitness(x)))
        .collect::<Vec<_>>();

    results.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (val, fit) in results {
        println!("Value {:?}, Fitness {:?}", val, fit);
    }
}
