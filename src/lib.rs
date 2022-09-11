use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use rayon::{iter::Either, prelude::*};

pub trait Organism {
    /// Calculate and return a fitness score using this.
    fn calculate_fitness(&self) -> f64;
    /// Combine the genetic content of the two parent args and make the self contain the resulting DNA.
    fn mate(&mut self, mate: &Self);
    /// Modify the DNA of the Organism randomly.
    fn mutate(&mut self, rng: &mut ThreadRng);
}

/// Calls `calculate_fitness`(), `mate`(), then `mutate`() accordingly to improve overall fitness.
pub fn evolve<T: Organism + Send + Sync>(input: &mut [T]) {
    let scores: Vec<f64> = input
        .par_iter()
        .map(Organism::calculate_fitness)
        .collect();

    let len = input.len();
    let mating_pool: Vec<bool> = (0..len)
        .into_par_iter()
        .map(|i| {
            let left = if i > 0 { i - 1 } else { len - 1 };
            let right = if i < len - 1 { i + 1 } else { 0 };

            scores[left] < scores[i] || scores[i] > scores[right]
        })
        .collect();

    let (mating_pool, mut replacing): (Vec<&T>, Vec<&mut T>) = input
        .par_iter_mut()
        .zip(mating_pool)
        .partition_map(|(member, mated)| {
            if mated {
                Either::Left(&*member)
            } else {
                Either::Right(member)
            }
        });

    replacing
        .par_iter_mut()
        .for_each_init(thread_rng, |rng, member| {
            if let Some(mate) = mating_pool.choose(rng) {
                member.mate(mate);
            }

            member.mutate(rng);
        });
}

/// Returns the best Organism struct from population.
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

/// Returns the average fitness score from population.
pub fn get_average<T: Organism + Send + Sync>(population: &[T]) -> f64 {
    population
        .par_iter()
        .map(Organism::calculate_fitness)
        .sum::<f64>()
        / (population.len() as f64)
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
