use crate::Organism;
use rand::*;
use std::fmt;

pub struct EvNum(pub f64);

// Organism whose content is soley just the fitness score.
impl Organism for EvNum {
    fn calculate_fitness(&self) -> f64 {
        self.0
    }

    fn mate(&mut self, mate: &EvNum) {
        self.0 = (mate.0 + self.0) / 2.0;
    }

    fn mutate(&mut self) {
        let mut rng = thread_rng();
        if rng.gen::<f64>() >= 0.2 {
            let max = 0.5;

            let value: f64 = rng.gen_range(-max..max);
            self.0 += value;
        }
    }
}

impl fmt::Display for EvNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
