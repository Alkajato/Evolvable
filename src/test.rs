use rand::{seq::SliceRandom, thread_rng, Rng};
use rayon::prelude::{
    FromParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::time::Instant;

use crate::{Evolvable, Evolver};

const ITERATIONS: std::ops::Range<i32> = 0..800;
const POP_SIZE: usize = 5_000_000; // Normal value is 5_000_000

#[derive(Debug)]
struct EvolvableNum(i32);
impl Evolvable for EvolvableNum {
    fn mate(&mut self, mate: &Self) {
        self.0 = ((self.0 as f32 + mate.0 as f32) / 2.0).round() as i32;
    }

    fn score(&self) -> f32 {
        self.0 as f32
    }

    fn mutate(&mut self) {
        self.0 += *[-1, 1].choose(&mut thread_rng()).unwrap();
    }
}

// Avg time: 0.02295171326658325
#[test]
fn profile() {
    let population: Vec<EvolvableNum> = Vec::from_par_iter(
        (0..POP_SIZE)
            .into_par_iter()
            .map(|_| EvolvableNum(thread_rng().gen_range(-50..50))),
    );

    let mut optimizer = Evolver::new(population);

    let mut total_time = 0.0;
    for count in ITERATIONS {
        let previous_average: f32 = optimizer
            .population
            .par_iter()
            .map(|x| x.0 as f32)
            .sum::<f32>()
            / POP_SIZE as f32;

        let start = Instant::now();
        optimizer.evolve();
        let time = start.elapsed().as_secs_f64();
        total_time += time;

        let current_average: f32 = optimizer
            .population
            .par_iter()
            .map(|x| x.0 as f32)
            .sum::<f32>()
            / POP_SIZE as f32;

        assert!(
            current_average > previous_average,
            "C_Avg: {}, P_Avg: {}",
            current_average,
            previous_average
        );

        eprint!(
            "\x1b[2KRun[{count}]: {time} \tAvg time: {:.17} \tAvg score: {current_average}\r",
            total_time / count as f64
        );
    }

    println!();
}

// Observe changes to fitness scores.
#[test]
fn observe() {
    let population = Vec::from_par_iter(
        (0..12)
            .into_par_iter()
            .map(|_| EvolvableNum(thread_rng().gen_range(-50..50))),
    );

    let mut optimizer = Evolver::new(population);

    loop {
        optimizer.evolve();
        eprint!("\x1b[2K{:?}\r", optimizer.population);

        std::thread::sleep(std::time::Duration::from_secs_f32(0.7));
    }
}
