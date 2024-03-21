use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum OutputFormats {
    Csv,
    Json,
    Plain,
}

#[derive(Parser)]
#[command(version, about, long_about)]
pub(crate) struct Cli {
    /// The CIDR block to check
    pub(crate) cidr: String,

    /// Show all IPs in CIDR block - if set to false, we only show the first and the last ip in the block
    #[arg(short, long, default_value_t = false)]
    pub(crate) all: bool,

    /// Additional toggle which has to be enabled - if set to false, we only show 2^20 (= 1.048.576) addresses
    #[arg(long, default_value_t = false)]
    pub(crate) danger_zone: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormats::Plain)]
    pub(crate) output: OutputFormats,
}
