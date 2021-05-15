use std::time::Instant;
use std::fs;

pub struct StopWatch {
    start: Instant,
    iterations: usize,
    average_time: f64,
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            start:  Instant::now(),
            iterations:  0,
            average_time:  -1.0
        }
    }

    /// Call to begin recording times.
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn lap(&mut self) {
        self.average_time = 
        if self.iterations <= 0 {
            self.start.elapsed().as_secs_f64()
        } else {
            self.average_time + self.start.elapsed().as_secs_f64()
        };

        self.iterations += 1;
    }

    pub fn stop(&mut self) {
        self.average_time /= self.iterations as f64;
    }

    pub fn make_results(&mut self, file_name: &str) -> std::io::Result<()> {
        self.stop();

        let results = format!("Average Time:\n{} seconds", self.average_time);
        fs::write(file_name, results.as_bytes())
    }
}