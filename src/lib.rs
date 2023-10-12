use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub trait Evolvable: Send + Sync {
    fn mate(&mut self, mate: &Self);
    fn score(&self) -> f32;
    fn mutate(&mut self);
}

pub struct Evolver<T: Evolvable> {
    pub scores: Vec<f32>,
    pub population: Vec<T>,
}

impl<T: Evolvable> Evolver<T> {
    pub fn new(population: Vec<T>) -> Self {
        let len = population.len();

        let min_safe_size = num_cpus::get() * 2;
        assert!(len >= min_safe_size, "Unexpectedly small population size may enable rare memory safety violations. Expected at least {min_safe_size}, got {len}");

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
    #[allow(invalid_reference_casting)]
    pub fn evolve(&mut self) {
        let len = self.scores.len();

        (0..len).into_par_iter().for_each(|i| {
            let left = if i > 0 { i - 1 } else { len - 1 };
            let right = if i < len - 1 { i + 1 } else { 0 };

            // Data race conditions should not occur if the user passed calling Evolver::new successfully.
            unsafe {
                let score = &self.scores[i];
                let score_left = &self.scores[left];
                let score_right = &self.scores[right];

                if score_left >= score && score <= score_right {
                    let mut mate = right;
                    if score_left > score_right {
                        mate = left;
                    }

                    // It is very unlikely two threads are close enough in the population Vec
                    // that them reading their own neighbors reads into content another thread is mutating.
                    let current = (&self.population[i]) as *const T as *mut T;
                    (*current).mate(&self.population[mate]);
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
