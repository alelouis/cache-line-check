use std::fs::File;
use std::io::Write;
use std::time::Instant;

const ARRAY_SIZE: usize = 8 * 1024 * 1024;
const REPETITIONS: usize = 50;

fn main() {
    let mut file = File::create("data.csv").expect("creation failed");
    let mut vec = vec![0; ARRAY_SIZE];

    for step in (1..128).step_by(1) {
        let start = Instant::now();

        // Performance loop
        for _ in 0..REPETITIONS {
            for i in (0..ARRAY_SIZE).step_by(step) {
                vec[i] *= 2;
            }
        }

        // Write to .csv
        let time_ms = start.elapsed().as_secs_f64() * 1_000.0;
        file.write(format!("{step},{time_ms:.05}\n").as_bytes())
            .expect("write failed");
    }
}
