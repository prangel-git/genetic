use genetic::*;

struct USize {
    value: usize,
}

impl Genetic for USize {
    fn choromosome(&self) -> Chromosome {
        Chromosome::from_element(self.value)
    }

    fn from_chromosome(chromosome: Chromosome) -> Self {
        if chromosome.is_empty() {
            USize{ value:0 }
        } else {
            let value  = chromosome.into_vec()[0];
            USize{ value }
        }
    }
}

fn fitness(x: &USize) -> usize {
    ((x.value >> 34 ) * ((usize::MAX - x.value) >> 34)).max(1)
}


fn main() {
    println!("min u {:}, fitness {:}", usize::MIN, fitness(&USize{ value:usize::MIN }));
    println!("max u {:}, fitness {:}", usize::MAX, fitness(&USize{ value:usize::MAX }));
    println!("half u {:}, fitness {:}", usize::MAX / 2, fitness(&USize{ value: usize::MAX / 2 }));

    let params = AlgorithmParams {
        rounds: 100,
        max_population: 20,
        mutation_rate: 0.1,
        co_rate: 0.01,
        fitness: Box::new(fitness),
    };

    let last_population = genetic_algorithm(&Vec::new(), &params);

    let mut results = last_population.iter().map(|x| (x.value, fitness(x))).collect::<Vec<_>>();

    results.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (val, fit) in results {
        println!("Value {:?}, Fitness {:?}", val, fit);
    }

}