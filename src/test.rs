use std::thread::current;

use rand::{thread_rng, Rng};

use crate::evolve;
use crate::par_evolve;
use crate::Organism;

use self::test_file_writer::StopWatch;
mod test_file_writer;
struct EvNum {
    fitness: f64,
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
        population.push(EvNum { fitness: 10 as f64 });
    }

    population
}

fn get_average(population: &[impl Organism]) -> f64 {
    let mut average = 0.0;
    for each in population {
        if average == 0.0 {
            average = each.calculate_fitness();
        }

        average += each.calculate_fitness();
    }

    average /= population.len() as f64;
    
    average
}

const ITERATIONS: std::ops::Range<i32> = 0..12;
const POP_SIZE: usize = 5_000_000;

#[test]
fn test_evolve() {
    // Testing if the code is actually functional.
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);
    
    let previous_average = get_average(&population);
    evolve(&mut population);
    let current_average = get_average(&population);

    assert!(current_average >= previous_average);

    // Benchmarking and writing to file.
    // TODO: Print average % improved each generation.
    let mut writer = StopWatch::new();

    for _ in ITERATIONS {
        writer.start();

        evolve(&mut population);

        writer.lap();

        assert!(population[POP_SIZE - 1].fitness > population[0].fitness);
    }

    match writer.make_results("time_evolve().txt") {
        Ok(_) => (),
        Err(err) => eprintln!("Could not write results file! {}", err),
    };
}

#[test]
fn test_par_evolve() {
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);
    
    let previous_average = get_average(&population);
    par_evolve(&mut population);
    let current_average = get_average(&population);

    assert!(current_average >= previous_average);

    // Benchmarking and writing to file.
    // TODO: Print average % improved each generation.
    let mut writer = StopWatch::new();

    for _ in ITERATIONS {
        writer.start();

        par_evolve(&mut population);

        writer.lap();

        assert!(population[POP_SIZE - 1].fitness > population[0].fitness);
    }

    match writer.make_results("time_par_evolve().txt") {
        Ok(_) => (),
        Err(err) => eprintln!("Could not write results file! {}", err),
    };
}
