use std::fs;
use std::time::Instant;

pub struct StopWatch {
    start: Instant,
    iterations: Vec<f64>,
    average_time: f64
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            start: Instant::now(),
            iterations: Vec::with_capacity(12),
            average_time: f64::NAN
        }
    }

    /// Call to begin recording times.
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    // Pushes time into vec of times.
    pub fn lap(&mut self) {
        self.iterations.push(self.start.elapsed().as_secs_f64());
    }

    /// Quits recording, creates the average time each lap took.
    pub fn stop(&mut self) {
        if self.iterations.len() == 0 {
            self.lap();
        }

        self.average_time = self.iterations.iter().sum::<f64>() / self.iterations.len() as f64;
    }

    pub fn string_results(&mut self) -> String {
        self.stop();

        format!("Average Time:\n{} seconds", self.average_time)
    }

    /// Writes data about the average time per lap to file.
    pub fn make_results(&mut self, file_name: &str, extra: Option<String>) {
        let output = if let Some(mut str) = extra {
            str.push_str(&format!("\n\n{}", self.string_results()));
            str
        } else {
            self.string_results()
        };

        if let Err(err) = fs::write(file_name, output.as_bytes()) {
            eprintln!("Could not write results file! {}", err);
        }
    }
}
