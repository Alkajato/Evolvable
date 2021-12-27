use rayon::prelude::*;

pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn mate(&mut self, mate: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self);
}

/// Returns the indices before and after index in items--loops around if out of bounds.
fn get_neighbors<T>(items: &[T], index: usize) -> (usize, usize) {
    let mut indices = (index - 1, index + 1);

    if index == 0 {
        indices.0 = items.len() - 1;
    }

    if index == items.len() - 1 {
        indices.1 = 0;
    }

    indices
}

/// Returns three references from items, left, center, and right of `items[index]`.
/// Loops around if `index == 0` or `index == items.len() - 1`
fn get_three<T>(items: &mut [T], index: usize) -> (&T, &mut T, &T) {
    if index == 0 {
        let (behind, ahead) = items.split_at_mut(1);

        (&ahead[ahead.len() - 1], &mut behind[0], &ahead[0])
    } else if index == items.len() - 1 {
        let (behind, ahead) = items.split_at_mut(index);

        (&behind[behind.len() - 1], &mut ahead[0], &behind[0])
    } else {
        let (behind, ahead) = items.split_at_mut(index);
        let (center, ahead) = ahead.split_at_mut(1);

        (&behind[behind.len() - 1], &mut center[0], &ahead[0])
    }
}

/// Calls calculate_fitness(), mate(), then mutate() accordingly to improve overall fitness.
pub fn evolve<T: Organism + Send + Sync>(input: &mut [T]) {
    // Make cores number of splits on input to process in parallel.
    let cores = num_cpus::get();

    let mut splits: Vec<&mut [T]> = vec![input];
    for _ in 0..cores / 2 {
        let mut temp = Vec::new();

        for each in splits {
            let (one, two) = each.split_at_mut(each.len() / 2);
            temp.push(one);
            temp.push(two);
        }

        splits = temp;
    }

    splits.par_iter_mut().for_each(|population| {
        let len = population.len();

        let scores: Vec<f64> = population
            .iter()
            .map(|item| item.calculate_fitness())
            .collect();

        let mated: Vec<bool> = scores
            .iter()
            .enumerate()
            .map(|(index, current_score)| {
                let (prev_score, next_score) = get_neighbors(&scores, index);

                if current_score <= &scores[prev_score] && current_score <= &scores[next_score] {
                    true
                } else {
                    false
                }
            })
            .collect();

        for i in 0..len {
            if mated[i] {
                let (before, after) = get_neighbors(&scores, i);

                let (behind, current, ahead) = get_three(*population, i);
                if scores[before] > scores[after] {
                    current.mate(behind);
                } else {
                    current.mate(ahead);
                }

                current.mutate();
            }
        }
    });
    // Due to splitting_at_mut, genes would not travel across the entire slice without rotation.
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
