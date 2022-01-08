use rayon::prelude::*;

pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn mate(&mut self, mate: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self);
}

/// Returns three references from input, left, center, and right of `items[index]`.
fn get_three<T>(input: &mut [T], index: usize) -> (&T, &mut T, &T) {
    if index == 0 {
        let (behind, ahead) = input.split_at_mut(1);

        return (&ahead[ahead.len() - 1], &mut behind[0], &ahead[0]);
    }

    if index == input.len() - 1 {
        let (behind, ahead) = input.split_at_mut(index);

        return (&behind[behind.len() - 1], &mut ahead[0], &behind[0]);
    }

    let (behind, ahead) = input.split_at_mut(index);
    let (center, ahead) = ahead.split_at_mut(1);

    return (&behind[behind.len() - 1], &mut center[0], &ahead[0]);
}

// Minimal amount of chunks that doesn't end with the last chunk being < 3
fn get_chunk_size(len: usize) -> usize {
    let cores = num_cpus::get();

    let mut chunk_size = len / cores;
    while len % chunk_size < 3 {
        if len % chunk_size == 0 && chunk_size >= 3 {
            return chunk_size;
        }

        chunk_size += 1;
    }

    chunk_size
}

#[test]
fn test_get_chunk_size() {
    let cores = num_cpus::get();

    for len in cores..=100 {
        let bools = vec![false; len];

        let chunk_size = get_chunk_size(bools.len());

        println!("\nSize: {} chunk_size: {}", len, chunk_size);
        for chunk in bools.chunks(chunk_size) {
            println!("\tSize of Chunk: {}", chunk.len());

            assert!(
                chunk.len() >= 3,
                "Size: {}. chunk_size (of {}) produced invalid chunk len: {}",
                len,
                chunk_size,
                chunk.len()
            );
        }
    }
}

/// Calls calculate_fitness(), mate(), then mutate() accordingly to improve overall fitness.
pub fn evolve<T: Organism + Send + Sync>(input: &mut [T]) {
    let mut scores: Vec<f64> = input
        .par_iter()
        .map(|item| item.calculate_fitness())
        .collect();

    // Split the input evenly to work on in parallel.
    let chunk_size = get_chunk_size(input.len());

    input
        .par_chunks_mut(chunk_size)
        .zip(scores.par_chunks_mut(chunk_size))
        .for_each(|(chunk, scores)| {
            let len = chunk.len();

            for i in 0..len {
                let (previous_score, current_score, next_score) = get_three(scores, i);

                if *current_score <= *previous_score && *current_score <= *next_score {
                    let (behind, current, ahead) = get_three(chunk, i);

                    if previous_score > next_score {
                        current.mate(behind);
                    } else {
                        current.mate(ahead);
                    }

                    current.mutate();
                }
            }
        });

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
