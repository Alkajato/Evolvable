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
    let mut mated: Vec<bool> = vec![false; population.len()];
    
    for i in 0..population.len() - 2 {
        if let [previous, current, next, ..] = &mut population[i..] {
            let current_score = current.calculate_fitness();

            if previous.calculate_fitness() >= current_score {
                current.mate(&previous);
                mated[i + 1] = true;
            } else if current_score <= next.calculate_fitness() {
                current.mate(&next);
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

    population.swap(0, population.len() - 2);
    population.swap(population.len() - 1, 1);
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
