use rand::{Rng, thread_rng};
use std::time::Instant;

mod organisms;
use organisms::*;

struct EvNum {
    fitness: f64
}

// Organism whose content is soley just the fitness score.
impl Organism for EvNum {
    fn calculate_fitness(&self) -> f64 {
        self.fitness
    }

    fn cross_over(&mut self, parent1: &EvNum, parent2: &EvNum) {
        self.fitness = (parent1.fitness + parent2.fitness) / 2.0;
    }

    fn mutate(&mut self) {
        let range = self.fitness * 2.0;

        let value = thread_rng().gen_range(-range..range);
        self.fitness += value;
    }
}

const POP_SIZE: usize = 5_000_000;
fn main() {    
    println!("Creating population......");
    let mut population: Vec<EvNum> = Vec::with_capacity(POP_SIZE);
    for _ in 0..POP_SIZE {
        population.push(EvNum {fitness: 10 as f64});
    }

    // This loop will run until it crashes the whole program.
    loop {
        let measure = Instant::now();
        par_evolve(&mut population);
        println!("Took {:?},  \t\t\tHighest Fitness: {}", measure.elapsed(), population[POP_SIZE-1].fitness);
    }
}
