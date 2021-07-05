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
    let len = population.len();
    let mut mated = vec![false; len];
    let scores: Vec<f64> = population
        .par_iter_mut()
        .map(|item| item.calculate_fitness())
        .collect();

    for i in 1..len - 1 {
        let before = i - 1;
        let after = i + 1;

        if scores[before] >= scores[i] && scores[i] <= scores[after] {
            if let [prev, current, next, ..] = &mut population[before..] {
                mated[i] = true;
                if scores[before] > scores[after] {
                    current.mate(&prev);
                } else {
                    current.mate(&next);
                }
            }
        }
    }

    population
        .par_iter_mut()
        .zip(mated)
        .for_each(|(item, mated)| {
            if mated {
                item.mutate();
            }
        });

    // We swap because otherwise the first and last member will be left out.
    population.swap(0, 1);
    population.swap(len - 1, len - 2);
}

/// Returns the best Organism struct from Population.
pub fn get_best<T: Organism>(population: &[T]) -> &T {
    population
        .iter()
        .reduce(|one, two| {
            if one.calculate_fitness() > two.calculate_fitness() {
                one
            } else {
                two
            }
        })
        .unwrap()
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
