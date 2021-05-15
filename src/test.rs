use rand::{Rng, thread_rng};

use crate::par_evolve;
use crate::Organism;
use crate::evolve;

use self::test_file_writer::StopWatch;
mod test_file_writer;
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

fn make_ev_nums(size: usize) -> Vec<EvNum> {
    let mut population: Vec<EvNum> = Vec::with_capacity(size);
    for _ in 0..size {
        population.push(EvNum {fitness: 10 as f64});
    }

    population
}

const ITERATIONS: std::ops::Range<i32> = 0..12;
const POP_SIZE: usize = 5_000_000;

#[test]
fn test_evolve() {
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);
    let mut writer = StopWatch::new();

    for _ in ITERATIONS {
        writer.start();

        evolve(&mut population);

        writer.lap();
        
        assert!(population[POP_SIZE-1].fitness > population[0].fitness);
    }

    match writer.make_results("time_evolve().txt") {
        Ok(_) => (),
        Err(err) => eprintln!("Could not write results file! {}", err)
    };
}

#[test]
fn test_par_evolve() {
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);
    let mut writer = StopWatch::new();

    for _ in ITERATIONS {
        writer.start();

        par_evolve(&mut population);

        writer.lap();

        assert!(population[POP_SIZE-1].fitness > population[0].fitness);
    }
    
    match writer.make_results("time_par_evolve().txt") {
        Ok(_) => (),
        Err(err) => eprintln!("Could not write results file! {}", err)
    };
}
