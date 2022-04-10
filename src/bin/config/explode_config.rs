use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct ExplodeConfig {

    /// Path to directory which will be flattened.
    #[clap(short, long)]
    pub directory: String
}