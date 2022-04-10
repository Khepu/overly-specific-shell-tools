use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct TruncateConfig {

    /// Path to directory which acts as the root of truncation
    #[clap(short, long)]
    pub directory: String,

    /// Depth at which to truncate
    #[clap(short, long, default_value_t = 0)]
    pub depth: u16
}
