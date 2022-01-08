use rand::{thread_rng, Rng};
use std::fmt;

use crate::evolve;
use crate::get_chunk_size;
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

    fn mate(&mut self, mate: &EvNum) {
        // self.fitness = mate.fitness;
        self.fitness = (mate.fitness + self.fitness) / 2.0;
    }

    fn mutate(&mut self) {
        let mut rng = thread_rng();
        if rng.gen::<f64>() >= 0.2 {
            let max = 0.5;

            let value: f64 = rng.gen_range(-max..max);
            self.fitness += value;
        }
    }
}

impl fmt::Display for EvNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fitness)
    }
}

fn make_ev_nums(size: usize) -> Vec<EvNum> {
    let mut population: Vec<EvNum> = Vec::with_capacity(size);

    for _ in 0..size {
        population.push(EvNum {
            fitness: thread_rng().gen::<f64>(),
        });
    }

    population
}

fn get_average(population: &[impl Organism]) -> f64 {
    let sum: f64 = population.iter().map(|item| item.calculate_fitness()).sum();
    sum / (population.len() as f64)
}

const ITERATIONS: std::ops::Range<i32> = 0..12;
const POP_SIZE: usize = 5_000_000; // Normal value is 5_000_000

#[test]
fn test_evolve() {
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);

    // population.iter().for_each(|member| print!("{} ", member));
    // println!("");

    let previous_average = get_average(&population);
    evolve(&mut population);
    let current_average = get_average(&population);

    // population.iter().for_each(|member| print!("{} ", member));
    // println!("");

    assert!(
        current_average > previous_average,
        "C_Avg: {}, P_Avg: {}",
        current_average,
        previous_average
    );

    let mut writer = StopWatch::new();
    for _ in ITERATIONS {
        writer.start();
        evolve(&mut population);
        writer.lap();
    }

    writer.make_results("time_evolve().txt");
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
