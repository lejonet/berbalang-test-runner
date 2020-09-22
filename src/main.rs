use std::fs::File;
use std::io::Read;

use berbalang_test_runner::TestOutline;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test_specification.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let test_specification: TestOutline = toml::from_str(&content).unwrap();

    berbalang_test_runner::run(test_specification);

    Ok(())
}
