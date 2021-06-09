use rayon::prelude::*;

pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn mate(&mut self, mate: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self);
}

/// Calls calculate_fitness(), mate(), then mutate() accordingly to improve overall fitness.
pub fn evolve<T: Organism + Send + Sync>(population: &mut [T]) {
    let scores = population
        .par_iter()
        .map(|item| item.calculate_fitness())
        .collect::<Vec<f64>>();
    let mut mated: Vec<bool> = vec![false; population.len()];

    for i in 0..population.len() - 2 {
        if let [previous, current, next, ..] = &mut population[i..] {
            if previous.calculate_fitness() >= scores[i + 1] {
                current.mate(previous);
                mated[i + 1] = true;
            } else if scores[i + 1] <= scores[i + 2] {
                current.mate(next);
                mated[i + 1] = true;
            }
        }
    }

    population
        .par_iter_mut()
        .zip(mated)
        .for_each(|(item, reproduced)| {
            if reproduced {
                item.mutate();
            }
        });
    
    population.swap(0, 1);
    population.swap(population.len() - 1, population.len() - 2);
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
