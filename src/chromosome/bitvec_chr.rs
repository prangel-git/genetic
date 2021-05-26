use bitvec::prelude::BitVec;

use bitvec::order::BitOrder;
use bitvec::store::BitStore;

use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

use super::Chromosome;

impl<LocalBits, Container> Chromosome for BitVec<LocalBits, Container> 
where 
LocalBits: BitOrder,
Container: BitStore,
{
    fn mutation(&self, mutation_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut mutated = self.clone();

        let d = Bernoulli::new(mutation_rate).unwrap();

        for gene in &mut mutated {
            if d.sample(&mut rng) {
                let new_value = !(*gene);
                gene.set(new_value);
            }
        }

        mutated
    }

    fn cross_over(&self, other: &Self, co_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut child = self.clone();

        let d = Bernoulli::new(co_rate).unwrap();

        for (gene, gene_other) in child.iter_mut().zip(other.clone()) {
            if d.sample(&mut rng) {
                gene.set(gene_other);
            }
        }

        child
    }
}