use serde::Deserialize;

use std::{fmt, thread, io};
use std::time::Duration;
use std::process::Command;
use std::sync::mpsc;

#[derive(Deserialize)]
pub struct TestSpecification {
    pub name: String,
    pub test_cmd: String,
    pub nr_of_test_runs: u8,
    pub test_length: u64,
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

    run_tests(test_outline);
}

fn run_tests(test_outline: TestOutline) {
    let mut create_container_args = vec!["copy", &test_outline.source_container, "placeholder-target-container"];
    for profile in &test_outline.container_profiles {
        create_container_args.push("-p");
        create_container_args.push(profile);
    }
    println!("{:#?}", create_container_args);
    for test in &test_outline.test_spec {
        let test_duration = Duration::new(test.test_length, 0);
        for test_nr in 0..test.nr_of_test_runs {
            let test_name = &format!("{}-{}", test.name, test_nr);
            let mut create_args = create_container_args.clone();
            create_args[2] = test_name;
            println!("{:#?}", create_args);
            println!("Copying {} to {}", test_outline.source_container, test_name);
            lxc(&create_args);
            println!("Starting {} container", test_name);
            lxc(&["start", test_name]);
            thread::sleep(Duration::new(5,0));
            println!("Executing command '{}' in {} container and letting it run for {} s", test.test_cmd, test_name, test.test_length);
            let (tx, rx) = mpsc::channel();
            tx.send((test_name.clone(), test.test_cmd.clone()));
            let test_thread = thread::spawn(move || {
                let (name, cmd) = rx.recv().unwrap();
                println!("This is {}, with cmd {}", name, cmd);
                let container_cmd = format!("echo {} >> thing", cmd);
                lxc(&["exec", &name, "--", &container_cmd]);
                //lxc(&["exec", test_name, "--", cmd])
            });
            test_thread.join().unwrap();
            lxc(&["stop", &test_name]);
        }
    }
}

fn lxc(args: &[&str]) -> io::Result<()> {
    let mut cmd = Command::new("lxc");
    cmd.args(args);

    let output = cmd.spawn()?.wait_with_output()?;
        println!("Output from command:\nstdout:\n{:#?}stderr:\n{:#?}", output.stdout, output.stderr);
        if output.status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("LXD {:?} failed with {}", args, output.status)
            ))
        }
}
