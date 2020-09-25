use serde::Deserialize;
pub mod berbalang_config;

use std::fs::File;
use std::io::{Read, Write};
use std::{fmt, thread, io};
use std::time::Duration;
use std::process::{Command, Stdio};
use std::sync::mpsc;

#[derive(Deserialize)]
pub struct TestSpecification {
    pub name: String,
    pub test_cmd: String,
    pub nr_of_test_runs: u8,
    pub test_length: Option<String>,
    pub path_config: String,
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
            writeln!(f, "Test cmd: {}, Amount of test runs: {}", test.test_cmd, test.nr_of_test_runs)?;
        }
        Ok(())
    }
}

pub fn run(test_outline: TestOutline) -> Result<(), Box<dyn std::error::Error>>{
    let mut total_nr_of_test_runs= 0u16;
    for test in &test_outline.test_spec {
       total_nr_of_test_runs += test.nr_of_test_runs as u16;
    }

    println!("{}\nTotal amount of tests: {}", test_outline, total_nr_of_test_runs);

    run_tests(test_outline)
}

fn create_berbalang_config(name: &str, test: &TestSpecification) -> io::Result<()> {
    let mut file = File::open(&test.path_config)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("Config file content:\n{}", content);
    let mut test_config: berbalang_config::Config = toml::from_str(&content).unwrap();
    if let Some(test_length) = test.test_length.clone() {
        test_config.timeout = Some(test_length);
    }

    file = File::create(format!("./{}.toml", name))?;
    content = toml::to_string(&test_config).unwrap();
    write!(file, "{}", content)?;

    Ok(())
}

fn run_tests(test_outline: TestOutline) -> Result<(), Box<dyn std::error::Error>> {
    let mut create_container_args = vec!["copy", &test_outline.source_container, "placeholder-target-container"];
    for profile in &test_outline.container_profiles {
        create_container_args.push("-p");
        create_container_args.push(profile);
    }
    println!("{:#?}", create_container_args);
    for test in &test_outline.test_spec {
        for test_nr in 0..test.nr_of_test_runs {
            let test_name = &format!("{}-{}", test.name, test_nr);

            create_berbalang_config(test_name, test)?;

            let mut create_args = create_container_args.clone();
            create_args[2] = test_name;
            println!("{:#?}", create_args);
            println!("Copying {} to {}", test_outline.source_container, test_name);
            lxc(&create_args)?;

            println!("Starting {} container", test_name);
            lxc(&["start", test_name])?;
            thread::sleep(Duration::new(5,0));

            println!("Pushing {}.toml to {} as ~/config.toml", test_name, test_name);
            lxc(&["file", "push", &format!("./{}.toml", test_name), &format!("{}/config.toml", test_name)])?;

            println!("Executing command '{}' in {} container", test.test_cmd, test_name);
            let (tx, rx) = mpsc::channel();
            tx.send((test_name.clone(), test.test_cmd.clone()))?;
            let test_thread = thread::spawn(move || {
                let (name, cmd) = rx.recv().unwrap();
                println!("This is {}, with cmd {}", name, cmd);
                let container_cmd = format!("echo {} >> thing", cmd);
                match lxc(&["exec", &name, "--", &container_cmd]) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Something went wrong when executing {} in {}: {}", cmd, name, e);
                        ()
                    }
                }
            });
            test_thread.join().unwrap();
            lxc(&["stop", &test_name])?;
        }
    }
    Ok(())
}

fn lxc(args: &[&str]) -> io::Result<()> {
    let output = Command::new("lxc")
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
        println!("Output from command:\nstdout:\n{}\nstderr:\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        if output.status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("LXD {:?} failed with {}", args, output.status)
            ))
        }
}
