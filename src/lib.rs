use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use rayon::prelude::*;

pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn mate(&mut self, mate: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self);
}

/// Iterates over input and calls calculate_fitness(), cross_over(), then mutate() accordingly to improve overall fitness.
pub fn evolve(population: &mut [impl Organism]) {
    let (bottom, top) = population.split_at_mut(population.len() / 2); // Continually splitting_at_mut was 3.07 times slower than splitting once.

    let mut rng = thread_rng();
    let range = Uniform::from(0..top.len()); // Using a range to sample from is approximately 13.43% faster.

    for elem in bottom {
        let mate = &top[range.sample(&mut rng)];

        elem.mate(mate);
        elem.mutate();
    }

    // Really want this to work with sort_by_cached_key instead.
    population.sort_unstable_by(|a, b| {
        a.calculate_fitness()
            .partial_cmp(&b.calculate_fitness())
            .unwrap()
    });
}

// Get par_evolve() to eventually use evolve(), test if evolve() is better than cellular genetic algorithm.
/// Multiple cores iterate over input and call calculate_fitness(), cross_over(), then mutate() accordingly to improve overall fitness.
pub fn par_evolve<T: Organism + Send + Sync>(population: &mut [T]) {
    let (bottom, top) = population.split_at_mut(population.len() / 2);
    let range = Uniform::from(0..top.len());

    bottom.into_par_iter().for_each_init(
        || rand::thread_rng(),
        |rng, elem| {
            // Parallel approach is 4.15~ times faster.
            let mate = &top[range.sample(rng)];

            elem.mate(mate);
            elem.mutate();
        },
    );

    // Sorting with parallelism is 4.1~ times faster than sequentially.
    population.par_sort_unstable_by(|a, b| {
        a.calculate_fitness()
            .partial_cmp(&b.calculate_fitness())
            .unwrap()
    });
}

// Learn pattern matching, find principle reason to use if let (matching one specific pattern actually)
/// For each member in population, compare to its neighbors' fitness scores. The best two out of the three replace the current.
pub fn cellular_evolve<T: Organism>(population: &mut [T]) {
    let scores = population
        .iter()
        .map(|element| element.calculate_fitness())
        .collect::<Vec<f64>>();

    for i in 0..population.len() - 2 {
        if let [previous, current, next] = &mut population[i..] {
            let mate = {
                if scores[i] > scores[i+2] {
                    previous
                } else {
                    next
                }
            };

            current.mate(mate);
            current.mutate();
        }
    }
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
