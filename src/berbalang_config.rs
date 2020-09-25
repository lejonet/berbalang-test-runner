use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Directly copied from https://github.com/oblivia-simplex/unicorn-rs/blob/master/libunicorn-sys/src/unicorn_const.rs
#[derive(Deserialize, Serialize)]
pub enum Arch {
    ARM = 1,
    ARM64,
    MIPS,
    X86,
    PPC,
    SPARC,
    M68K,
}

// Directly copied from https://github.com/oblivia-simplex/unicorn-rs/blob/master/libunicorn-sys/src/unicorn_const.rs
#[derive(Deserialize, Serialize)]
pub enum Mode {
    LITTLE_ENDIAN = 0,
    MODE_16 = 1 << 1,
    MODE_32 = 1 << 2,
    MODE_64 = 1 << 3,
    THUMB = 1 << 4,
    MCLASS = 1 << 5,
    V8 = 1 << 6,
    BIG_ENDIAN = 1 << 30,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub job: Job,
    pub timeout: Option<String>,
    pub selection: Selection,
    pub num_islands: Option<usize>,
    pub island_id: Option<usize>,
    pub crossover_period: f64,
    pub crossover_algorithm: Option<String>,
    pub crossover_rate: f32,
    pub data: Option<DataConfig>,
    pub max_init_len: usize,
    pub max_length: usize,
    pub min_init_len: usize,
    pub mutation_rate: Option<f64>,
    pub mutation_exponent: f64,
    pub observer: ObserverConfig,
    pub pop_size: usize,
    pub problems: Option<Vec<ClassificationProblem>>,
    pub roulette: Option<RouletteConfig>,
    pub tournament: Option<TournamentConfig>,
    pub roper: Option<RoperConfig>,
    pub linear_gp: Option<LinearGpConfig>,
    pub hello: Option<HelloConfig>,
    pub num_epochs: usize,
    pub fitness: FitnessConfig,
    pub random_seed: Option<u64>,
    pub push_vm: Option<PushVm>,
}

#[derive(Deserialize, Serialize)]
pub struct DataConfig {
    pub path: String,
}

#[derive(Deserialize, Serialize)]
pub enum Job {
    Roper,
    Hello,
    LinearGp,
}

#[derive(Deserialize, Serialize)]
pub struct PushVm {
    pub max_steps: Option<usize>,
    pub min_len: usize,
    pub max_len: usize,
    pub literal_rate: f64,
}

#[derive(Deserialize, Serialize)]
pub struct FitnessConfig {
    pub target: f64,
    pub eval_by_case: bool,
    pub dynamic: bool,
    priority: Option<String>,
    pub function: String,
    pub weighting: String,
}

#[derive(Deserialize, Serialize)]
pub struct TournamentConfig {
    pub tournament_size: Option<usize>,
    pub geographic_radius: usize,
    pub migration_rate: f64,
    pub num_offspring: usize,
    pub num_parents: usize,
}

#[derive(Deserialize, Serialize)]
pub struct ObserverConfig {
    pub dump_population: bool,
    pub dump_soup: bool,
    pub window_size: usize,
    pub report_every: Option<usize>,
    pub dump_every: Option<usize>,
    pub full_data_directory: Option<String>,
    data_directory: String,
    pub population_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct HelloConfig {
    pub target: String,
}

#[derive(Deserialize, Serialize)]
pub struct LinearGpConfig {
    pub max_steps: usize,
    pub num_registers: Option<usize>,
    pub return_registers: Option<usize>,
}

#[derive(Deserialize, Serialize)]
pub struct RoperConfig {
    pub use_push: bool,
    pub gadget_file: Option<String>,
    pub output_registers: Option<Vec<String>>,
    pub input_registers: Option<Vec<String>>,
    pub randomize_registers: Option<bool>,
    pub register_pattern_file: Option<String>,
    pub parsed_register_patterns: Option<Vec<RegisterPattern>>,
    pub soup: Option<Vec<u64>>,
    pub soup_size: Option<usize>,
    pub arch: Option<Arch>,
    pub mode: Option<Mode>,
    pub num_workers: Option<usize>,
    pub num_emulators: Option<usize>,
    pub wait_limit: Option<u64>,
    pub max_emu_steps: Option<usize>,
    pub millisecond_timeout: Option<u64>,
    pub record_basic_blocks: Option<bool>,
    pub record_memory_writes: Option<bool>,
    pub emulator_stack_size: Option<usize>,
    pub binary_path: String,
    pub ld_paths: Option<Vec<String>>,
    pub bad_bytes: Option<HashMap<String, u8>>,
    pub memory_pattern: Option<Vec<u8>>,
    pub break_on_calls: Option<bool>,
    pub monitor_stack_writes: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct ClassificationProblem {
    pub input: Vec<i32>,
    pub output: i32,
    pub tag: u64,
}


#[derive(Deserialize, Serialize)]
pub enum Selection {
    Tournament,
    Roulette,
    Metropolis,
    Lexicase,
}

#[derive(Deserialize, Serialize)]
pub enum Problem {
    Classification(ClassificationProblem),
    RegisterSpecification(RegisterPattern),
    MemoryPattern(Vec<u8>),
}

#[derive(Deserialize, Serialize)]
pub struct RouletteConfig {
    pub weight_decay: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterValue {
    pub vals: Vec<u64>, // Alternatives
    pub deref: usize,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterPattern(pub HashMap<String, RegisterValue>);