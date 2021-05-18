use std::fs;
use std::time::Instant;

pub struct StopWatch {
    start: Instant,
    iterations: usize,
    average_time: f64,
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            start: Instant::now(),
            iterations: 0,
            average_time: -1.0,
        }
    }

    /// Call to begin recording times.
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    /// Records another "iteration" into the average.
    pub fn lap(&mut self) {
        self.average_time = if self.iterations <= 0 {
            self.start.elapsed().as_secs_f64()
        } else {
            self.average_time + self.start.elapsed().as_secs_f64()
        };

        self.iterations += 1;
    }

    /// Quits recording, creates the average time each lap took.
    pub fn stop(&mut self) {
        self.average_time /= self.iterations as f64;
    }

    /// Writes data about the average time per lap to file.
    pub fn make_results(&mut self, file_name: &str) {
        self.stop();

        let results = format!("Average Time:\n{} seconds", self.average_time);

        match fs::write(file_name, results.as_bytes()) {
            Ok(_) => (),
            Err(err) => eprintln!("Could not write results file! {}", err),
        };
    }
}
