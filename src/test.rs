use rand::{thread_rng, Rng};
use rayon::prelude::{
    FromParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::time::Instant;

use crate::evolve;

const ITERATIONS: std::ops::Range<i32> = 0..50;
const POP_SIZE: usize = 5_000_000; // Normal value is 5_000_000

#[test]
fn profile() {
    let mut population = Vec::from_par_iter(
        (0..POP_SIZE)
            .into_par_iter()
            .map(|_| thread_rng().gen_range(-50..50)),
    );

    let score = |x: &i32| *x;
    let cross_over = |x: &mut i32, y: &i32| *x = ((*x as f32 + *y as f32) / 2.0).round() as i32;
    let mutate = |x: &mut i32| unsafe {
        let nums = [-1, 1];
        *x += nums.get_unchecked(thread_rng().gen_range(0..nums.len()));
    };

    let mut total_time = 0.0;
    for count in ITERATIONS {
        let previous_average: f32 =
            population.par_iter().map(|x| *x as f32).sum::<f32>() / POP_SIZE as f32;

        let start = Instant::now();
        evolve(&mut population, score, cross_over, mutate);
        let time = start.elapsed().as_secs_f64();
        total_time += time;

        let current_average: f32 =
            population.par_iter().map(|x| *x as f32).sum::<f32>() / POP_SIZE as f32;

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
    let mut population = Vec::from_par_iter(
        (0..10)
            .into_par_iter()
            .map(|_| thread_rng().gen_range(-50..50)),
    );

    let score = |x: &i32| *x;
    let cross_over = |x: &mut i32, y: &i32| *x = ((*x as f32 + *y as f32) / 2.0).round() as i32;
    let mutate = |x: &mut i32| unsafe {
        let nums = [-1, 1];
        *x += nums.get_unchecked(thread_rng().gen_range(0..nums.len()));
    };

    for _ in 0.. {
        evolve(&mut population, score, cross_over, mutate);
        eprint!("\x1b[2K{population:?}\r");

        std::thread::sleep(std::time::Duration::from_secs_f32(0.7));
    }
}
