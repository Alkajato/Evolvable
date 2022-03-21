use self::test_file_writer::StopWatch;
use crate::test::test_types::*;
use crate::{evolve, get_average};

mod test_file_writer;
mod test_types;

const ITERATIONS: std::ops::Range<i32> = 0..24;
const POP_SIZE: usize = 5_000_000; // Normal value is 5_000_000

#[test]
fn test_evolve() {
    let mut population: Vec<EvNum> = make_ev_nums(POP_SIZE);
    // let mut population: Vec<i32> = vec![0; POP_SIZE];

    let mut improvement = 0.0;
    let mut writer = StopWatch::new();
    for _ in ITERATIONS {
        let previous_average = get_average(&population);

        writer.start();
        evolve(&mut population);
        writer.lap();

        let current_average = get_average(&population);

        assert!(
            current_average > previous_average,
            "C_Avg: {}, P_Avg: {}",
            current_average,
            previous_average
        );

        let percent_improved = ((current_average / previous_average) * 100.0) - 100.0;
        improvement += percent_improved;
    }

    improvement = improvement / ITERATIONS.end as f64;
    let average_improvement = String::from(format!("Average Improvement:\n{improvement}%"));

    writer.make_results("time_evolve().txt", Some(average_improvement));
}
