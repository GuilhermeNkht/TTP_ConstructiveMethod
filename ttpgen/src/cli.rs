use clap::Parser;

/// Command-line interface for TTP Solution Generator.
#[derive(Parser, Debug)]
#[command(name = "ttpgen", version = "1.01", about = "Generates TTP schedules")]
pub struct Cli {
    /// Path to the XML instance file
    #[arg(long = "input")]
    pub input: String,

    /// Directory to save generated solutions
    #[arg(long = "output-solutions", default_value = "solutions_output")]
    pub output_solutions: String,

    /// Directory to save generated permutations
    #[arg(long = "output-permutations", default_value = "perms_output")]
    pub output_permutations: String,

    /// Number of random permutations to generate
    #[arg(long = "permutations", default_value_t = 10)]
    pub permutations: i32,

    /// Random seed for reproducibility
    #[arg(long = "seed", default_value_t = 42)]
    pub seed: u64,

    /// Disable saving to disk
    #[arg(long = "save", default_value_t = false)]
    pub save: bool,

    /// Enable or disable logging
    #[arg(long = "log", default_value_t = false)]
    pub log_enabled: bool,
}
