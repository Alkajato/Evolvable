use rand::{thread_rng, Rng};

use crate::evolve;
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
        self.fitness = (mate.fitness + self.fitness) / 2.0;
    }

    fn mutate(&mut self) {
        let max = 50.0;

        let value: f64 = thread_rng().gen_range(-max..max);
        self.fitness += value;
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
    let average: f64 = population.iter().map(|item| item.calculate_fitness()).sum();
    average / (population.len() as f64)
}

const ITERATIONS: std::ops::Range<i32> = 0..12;
const POP_SIZE: usize = 5_000_000;

#[test]
fn test_evolve() {
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);

    let previous_average = get_average(&population);
    evolve(&mut population);
    let current_average = get_average(&population);

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
