use rand::{Rng, thread_rng};
use rayon::prelude::*;
        
pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn cross_over(&mut self, parent1: &Self, parent2: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self);
}

/// Iterates over input and calls calculate_fitness(), cross_over(), then mutate() accordingly to improve overall fitness.
pub fn evolve(population: &mut [impl Organism]) {
    let (bottom, top) = population.split_at_mut(population.len() / 2); // Continually splitting_at_mut was 3.07 times slower than splitting once.
    let mut rng = thread_rng();

    for elem in bottom {
        let parent1 = &top[rng.gen_range(0..top.len())];
        let parent2 = &top[rng.gen_range(0..top.len())];

        elem.cross_over(parent1, parent2);
        elem.mutate();
    }
        
    // Really want this to work with sort_by_cached_key instead.
    population.sort_unstable_by(|a, b| a.calculate_fitness().partial_cmp(&b.calculate_fitness()).unwrap());
}

/// Multiple cores iterate over input and call calculate_fitness(), cross_over(), then mutate() accordingly to improve overall fitness.
pub fn par_evolve<T: Organism + Send + Sync>(population: &mut [T]) { // Approximately 5~ times faster than sequential.
    let (bottom, top) = population.split_at_mut(population.len() / 2);

    bottom.into_par_iter().for_each_init(|| rand::thread_rng(), |rng, elem| { // Parallel approach is 4.15~ times faster.
        let parent1 = &top[rng.gen_range(0..top.len())];
        let parent2 = &top[rng.gen_range(0..top.len())];

        elem.cross_over(parent1, parent2);
        elem.mutate();
    });

    // Sorting with parallelism is 4.1~ times faster than sequentially.
    population.par_sort_unstable_by(|a, b| a.calculate_fitness().partial_cmp(&b.calculate_fitness()).unwrap());
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use crate::par_evolve;
    use rayon::prelude::*;
    use crate::Organism;
    use crate::evolve;




    struct EvNum {
        fitness: f64
    }

    // Organism whose content is soley just the fitness score.
    impl Organism for EvNum {
        fn calculate_fitness(&self) -> f64 {
            self.fitness
        }

        fn cross_over(&mut self, parent1: &EvNum, parent2: &EvNum) {
            self.fitness = (parent1.fitness + parent2.fitness) / 2.0;
        }

        fn mutate(&mut self) {
            let range = self.fitness * 2.0;

            let value = thread_rng().gen_range(-range..range);
            self.fitness += value;
        }
    }

    const ITERATIONS: std::ops::Range<i32> = 0..5;
    const POP_SIZE: usize = 5_000_000;

    #[test]
    fn test_evolve() {
        let mut population: Vec<EvNum> = Vec::with_capacity(POP_SIZE);
        for _ in 0..POP_SIZE {
            population.push(EvNum {fitness: 10 as f64});
        }

        for _ in ITERATIONS {
            evolve(&mut population);
            assert!(population[POP_SIZE-1].fitness > population[0].fitness);
        }
    }

    #[test]
    fn test_par_evolve() {
        let mut population: Vec<EvNum> = Vec::with_capacity(POP_SIZE);
        for _ in 0..POP_SIZE {
            population.push(EvNum {fitness: 10 as f64});
        }

        for _ in ITERATIONS {
            par_evolve(&mut population);
            assert!(population[POP_SIZE-1].fitness > population[0].fitness);
        }        
    }
}
