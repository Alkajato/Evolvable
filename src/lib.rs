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
    let mut mated: Vec<bool> = vec![false; len];

    let mut work = |index: usize, behind: &mut T, current: &mut T, after: &mut T| {
        let current_score = current.calculate_fitness();

        if behind.calculate_fitness() >= current_score {
            current.mate(&behind);
            mated[index] = true;
        } else if current_score <= after.calculate_fitness() {
            current.mate(&after);
            mated[index] = true;
        }
    };

    // The first member mates with member infront, and the last member.
    if let [current, second, .., last] = &mut population[..] {
        work(0, last, current, second);
    }

    for i in 1..len - 1 {
        if let [previous, current, next, ..] = &mut population[i..] {
            work(i, previous, current, next);
        }
    }

    // The last member mates with member behind, and the first member.
    if let [first, .., behind, current] = &mut population[..] {
        work(len - 1, behind, current, first);
    }

    population
        .par_iter_mut()
        .zip(mated)
        .for_each(|(item, reproduced)| {
            if reproduced {
                item.mutate();
            }
        });
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
