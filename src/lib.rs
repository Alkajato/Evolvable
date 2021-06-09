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
        .map(|element| element.calculate_fitness())
        .collect::<Vec<f64>>();
    let mut mated: Vec<bool> = vec![false; population.len()];

    for i in 0..population.len() - 2 {
        if let [previous, current, next, ..] = &mut population[i..] {
            let mut state = false;

            if scores[i] > scores[i + 1] {
                current.mate(previous);
                state = true;
            } else if scores[i + 1] < scores[i + 2] {
                current.mate(next);
                state = true;
            }

            mated[i + 1] = state;
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
