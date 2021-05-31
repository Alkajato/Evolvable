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

/// Iterates over input and calls calculate_fitness(), mate(), then mutate() accordingly to improve overall fitness.
pub fn evolve<T: Organism + Send + Sync>(population: &mut [T]) {
    let scores = population
        .iter()
        .map(|element| element.calculate_fitness())
        .collect::<Vec<f64>>();

    for i in 0..population.len() - 2 {
        if let [previous, current, next, ..] = &mut population[i..] {
            let mate = {
                if scores[i] > scores[i+2] {
                    previous
                } else {
                    next
                }
            };
            
            current.mate(mate);
        }
    }

    population
        .par_iter_mut()
        .for_each(|item| item.mutate());
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
