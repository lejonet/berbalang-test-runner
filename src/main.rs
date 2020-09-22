use serde::Deserialize;

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

#[derive(Deserialize)]
struct TestSpecification {
    pub test_cmd: String,
    pub nr_of_test_runs: u8,
    pub test_length: u16,
}

#[derive(Deserialize)]
struct TestOutline {
    pub source_container: String,
    pub test_spec: Vec<TestSpecification>,
}

impl fmt::Display for TestOutline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        writeln!(f, "Source container: {}\nTests: ", self.source_container)?;
        for test in &self.test_spec {
            writeln!(f, "Test cmd: {}, Test length: {}, Amount of test runs: {}", test.test_cmd, test.test_length, test.nr_of_test_runs)?;
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("test_specification.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let test_specification: TestOutline = toml::from_str(&content).unwrap();

    let mut total_nr_of_test_runs= 0u16;
    let mut total_run_time = 0u32;
    for test in &test_specification.test_spec {
       total_nr_of_test_runs += test.nr_of_test_runs as u16;
       total_run_time += test.test_length as u32 * test.nr_of_test_runs as u32;
    }

    println!("{}\nTotal amount of tests: {} Total run time: {} h", test_specification, total_nr_of_test_runs, total_run_time as f64/3600f64);

    Ok(())
}
