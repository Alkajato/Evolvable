use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::iter::*;

use self::test_file_writer::StopWatch;
use crate::evolve;
use crate::get_chunk_size;
use crate::Organism;
use crate::test::test_types::*;

mod test_types;
mod test_file_writer;

fn get_average<T: Organism + Send + Sync>(population: &[T]) -> f64 {
    let sum: f64 = population
        .par_iter()
        .map(|item| item.calculate_fitness())
        .sum();
    sum / (population.len() as f64)
}

const ITERATIONS: std::ops::Range<i32> = 0..24;
const POP_SIZE: usize = 5_000_000; // Normal value is 5_000_000

#[test]
fn test_evolve() {
    let algorithm = evolve;

    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);

    let previous_average = get_average(&population);
    algorithm(&mut population);
    let current_average = get_average(&population);

    assert!(
        current_average > previous_average,
        "C_Avg: {}, P_Avg: {}",
        current_average,
        previous_average
    );

    let mut improvement = 0.0;
    let mut writer = StopWatch::new();
    for _ in ITERATIONS {
        let previous_average = get_average(&population);

        writer.start();
        algorithm(&mut population);
        writer.lap();

        let current_average = get_average(&population);

        let percent_improved = ((current_average / previous_average) * 100.0) - 100.0;
        improvement += percent_improved;
    }

    improvement = improvement / ITERATIONS.end as f64;
    let average_improvement = String::from(format!("Average Improvement:\n{improvement}%"));

    writer.make_results("time_evolve().txt", Some(average_improvement));
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

fn evolve_new<T: Organism + Send + Sync>(input: &mut [T]) {
    let scores: Vec<f64> = input
        .par_iter()
        .map(|item| item.calculate_fitness())
        .collect();

    let len = scores.len();
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

    replacing.par_iter_mut().for_each_init(
        || thread_rng(),
        |rng, member| {
            if let Some(mate) = mating_pool.choose(rng) {
                member.mate(&mate);
            }

            member.mutate();
        },
    );
}