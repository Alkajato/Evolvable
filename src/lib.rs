use rayon::prelude::*;

pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn mate(&mut self, mate: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self);
}

/// Returns the indices before and after index in input--loops around if out of bounds.
fn get_neighbors<T>(input: &[T], index: usize) -> (usize, usize) {
    if index == 0 {
        return (input.len() - 1, index + 1);
    }

    if index == input.len() - 1 {
        return (index - 1, 0);
    }

    (index - 1, index + 1)
}

/// Returns three references from input, left, center, and right of `items[index]`.
fn get_three<T>(input: &mut [T], index: usize) -> (&T, &mut T, &T) {
    if index == 0 {
        let (behind, ahead) = input.split_at_mut(1);

        (&ahead[ahead.len() - 1], &mut behind[0], &ahead[0])
    } else if index == input.len() - 1 {
        let (behind, ahead) = input.split_at_mut(index);

        (&behind[behind.len() - 1], &mut ahead[0], &behind[0])
    } else {
        let (behind, ahead) = input.split_at_mut(index);
        let (center, ahead) = ahead.split_at_mut(1);

        (&behind[behind.len() - 1], &mut center[0], &ahead[0])
    }
}

/// Calls calculate_fitness(), mate(), then mutate() accordingly to improve overall fitness.
pub fn evolve<T: Organism + Send + Sync>(input: &mut [T]) {
    // Do the work that can be work-stealed easily using rayon.
    let scores: Vec<f64> = input
        .par_iter()
        .map(|item| item.calculate_fitness())
        .collect();

    // Split the input evenly to work on in parallel.
    let chunk_size = input.len() / num_cpus::get();

    input
        .par_chunks_mut(chunk_size)
        .zip(scores.par_chunks(chunk_size))
        .for_each(|(chunk, scores)| {
            let len = chunk.len();

            for i in 0..len {
                let (before, after) = get_neighbors(&scores, i);

                let (behind, current, ahead) = get_three(chunk, i);
                if scores[before] > scores[after] {
                    current.mate(behind);
                } else {
                    current.mate(ahead);
                }
            }
        });

    // Do mutations using rayon to take advantage of the work-stealing algorithm.
    input.par_iter_mut().for_each(|item| item.mutate());

    // Since iterating over chunks do not overlap, genes would not travel across the entire slice without rotation.
    input.rotate_left(1);
}

/// Returns the best Organism struct from Population.
pub fn get_best<T: Organism + Send + Sync>(population: &[T]) -> &T {
    population
        .par_iter()
        .reduce_with(|one, two| {
            if one.calculate_fitness() > two.calculate_fitness() {
                one
            } else {
                two
            }
        })
        .expect("Population failed to yield best from get_best()")
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
