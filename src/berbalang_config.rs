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

impl Default for Arch {
    fn default() -> Self { Arch::X86 }
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

impl Default for Mode {
    fn default() -> Self { Mode::MODE_32 }
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub job: Job,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    pub selection: Selection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_islands: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub island_id: Option<usize>,
    pub crossover_period: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossover_algorithm: Option<String>,
    pub crossover_rate: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DataConfig>,
    pub max_init_len: usize,
    pub max_length: usize,
    pub min_init_len: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_rate: Option<f64>,
    pub mutation_exponent: f64,
    pub observer: ObserverConfig,
    pub pop_size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problems: Option<Vec<ClassificationProblem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roulette: Option<RouletteConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament: Option<TournamentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roper: Option<RoperConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_gp: Option<LinearGpConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hello: Option<HelloConfig>,
    pub num_epochs: usize,
    pub fitness: FitnessConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<String>,
    pub function: String,
    pub weighting: String,
}

#[derive(Deserialize, Serialize)]
pub struct TournamentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_every: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dump_every: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_data_directory: Option<String>,
    data_directory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct HelloConfig {
    pub target: String,
}

#[derive(Deserialize, Serialize)]
pub struct LinearGpConfig {
    pub max_steps: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_registers: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_registers: Option<usize>,
}

#[derive(Deserialize, Serialize)]
pub struct RoperConfig {
    pub use_push: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gadget_file: Option<String>,
    #[serde(default)]
    pub output_registers: Vec<String>,
    #[serde(default)]
    pub input_registers: Vec<String>,
    #[serde(default)]
    pub randomize_registers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_pattern_file: Option<String>,
    #[serde(skip)]
    pub parsed_register_patterns: Vec<RegisterPattern>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soup: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soup_size: Option<usize>,
    #[serde(default)]
    pub arch: Arch,
    #[serde(default)]
    pub mode: Mode,
    #[serde(default)]
    pub num_workers: usize,
    #[serde(default)]
    pub num_emulators: usize,
    #[serde(default)]
    pub wait_limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_emu_steps: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millisecond_timeout: Option<u64>,
    #[serde(default)]
    pub record_basic_blocks: bool,
    #[serde(default)]
    pub record_memory_writes: bool,
    #[serde(default)]
    pub emulator_stack_size: usize,
    pub binary_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ld_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_bytes: Option<HashMap<String, u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_pattern: Option<Vec<u8>>,
    #[serde(default)]
    pub break_on_calls: bool,
    #[serde(default)]
    pub monitor_stack_writes: bool,
}

impl Default for RoperConfig {
    fn default() -> Self {
        Self {
            use_push: false,
            gadget_file: None,
            output_registers: vec![],
            input_registers: vec![],
            randomize_registers: false,
            register_pattern_file: None,
            parsed_register_patterns: vec![],
            soup: None,
            soup_size: None,
            arch: Arch::X86,
            mode: Mode::MODE_64,
            memory_pattern: None,
            num_workers: 8,
            num_emulators: 8,
            wait_limit: 500,
            max_emu_steps: Some(0x10_000),
            millisecond_timeout: Some(500),
            record_basic_blocks: false,
            record_memory_writes: false,
            emulator_stack_size: 0x1000,
            binary_path: "/bin/sh".to_string(),
            ld_paths: None,
            bad_bytes: None,
            break_on_calls: false,
            monitor_stack_writes: false,
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_decay: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterValue {
    pub vals: Vec<u64>, // Alternatives
    pub deref: usize,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterPattern(pub HashMap<String, RegisterValue>);