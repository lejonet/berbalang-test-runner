use serde::Deserialize;

use std::fmt;
use std::time::Duration;

#[derive(Deserialize)]
pub struct TestSpecification {
    pub test_cmd: String,
    pub nr_of_test_runs: u8,
    pub test_length: u16,
}

#[derive(Deserialize)]
pub struct TestOutline {
    pub source_container: String,
    pub container_profiles: Vec<String>,
    pub test_spec: Vec<TestSpecification>,
}

impl fmt::Display for TestOutline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        writeln!(f, "Source container: {}\nContainer profiles to apply: ", self.source_container)?;
        for profile in &self.container_profiles {
            writeln!(f, "{}", profile)?;
        }
        writeln!(f, "Tests: ")?;
        for test in &self.test_spec {
            writeln!(f, "Test cmd: {}, Test length: {}, Amount of test runs: {}", test.test_cmd, test.test_length, test.nr_of_test_runs)?;
        }
        Ok(())
    }
}

pub fn run(test_outline: TestOutline) {
    let mut total_nr_of_test_runs= 0u16;
    let mut total_run_time = 0u32;
    for test in &test_outline.test_spec {
       total_nr_of_test_runs += test.nr_of_test_runs as u16;
       total_run_time += test.test_length as u32 * test.nr_of_test_runs as u32;
    }

    println!("{}\nTotal amount of tests: {} Total run time: {} h", test_outline, total_nr_of_test_runs, total_run_time as f64/3600f64);
}
