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
        .par_iter()
        .map(|element| element.calculate_fitness())
        .collect::<Vec<f64>>();
    let average: f64 = scores.iter().sum::<f64>() / population.len() as f64;

    for i in 0..population.len() - 2 {
        if let [previous, current, next, ..] = &mut population[i..] {
            if scores[i + 1] <= average { // Only replace if current is under or equal average.
                current.mate(
                    if scores[i] > scores[i + 2] {
                        previous
                    } else {
                        next
                    }
                );
            }
        }
    }

    population
        .par_iter_mut()
        .zip(scores)
        .for_each(|(item, score)| {
            if score <= average {
                item.mutate();
            }
        });
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
