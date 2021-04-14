use rand::{Rng, thread_rng};
use rayon::prelude::*;
        
pub trait Organism {
    /// Calculate and return a fitness score using this. This function may be called multiple times so returning a field of the result cached is a good idea.
    fn calculate_fitness(&self) -> f64;
    fn cross_over(&mut self, parent1: &Self, parent2: &Self);
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
