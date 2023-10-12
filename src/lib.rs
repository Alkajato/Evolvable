use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub trait Evolvable: Send + Sync {
    fn mate(&mut self, mate: &Self);
    fn score(&self) -> f32;
    fn mutate(&mut self);
}

pub struct Evolver<T: Evolvable> {
    pub scores: Vec<f32>,
    population: Vec<T>,
}

impl<T: Evolvable> Evolver<T> {
    pub fn new(population: Vec<T>) -> Self {
        let cpu_count = num_cpus::get();
        let len = population.len();

        let min_safe_size = cpu_count * 2;
        assert!(len >= min_safe_size, "Unexpectedly small population size may enable rare memory safety violations. Expected at least more than {min_safe_size}, got {len}");

        let scores = population.par_iter().map(Evolvable::score).collect();
        Evolver { scores, population }
    }

    pub fn len(&self) -> usize {
        self.population.len()
    }

    pub fn is_empty(&self) -> bool {
        self.population.len() == 0
    }

    /// Parallel naiive genetic algorithm that uses crossover and mutation operators.
    /// Cross overs mutate the first arg to cross over with the read-only reference arg.
    #[allow(invalid_reference_casting)]
    pub fn evolve(&mut self) {
        let len = self.scores.len();

        (0..len).into_par_iter().for_each(|i| {
            let left = if i > 0 { i - 1 } else { len - 1 };
            let right = if i < len - 1 { i + 1 } else { 0 };

            unsafe {
                let score = &self.scores[i];
                let score_left = &self.scores[left];
                let score_right = &self.scores[right];

                if score_left >= score && score <= score_right {
                    let mut mate = right;
                    if score_left > score_right {
                        mate = left;
                    }

                    // If one neighbor is worse then population[i] is not mut referenced.
                    // So at any given index if we mutate we do not have writes happening to our neighbors.
                    // Therefore this is safe across threads because at worst multiple threads will only read-access the same thing.
                    let current = (&self.population[i]) as *const T as *mut T;
                    (*current).mate(self.population.get_unchecked(mate));
                    (*current).mutate();

                    let ptr = score as *const f32 as *mut f32;
                    *ptr = (*current).score();
                }
            }
        });
    }
}

// Referring to test.rs for separate tests file.
#[cfg(test)]
mod test;
