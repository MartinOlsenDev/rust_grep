use clap::Parser;

//todo: Add this to main when arguments are required. Current implementation is temporary solution.
//todo: Use builder pattern to make positional arguments rather than named ones.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Pattern to match on
    #[arg(short, long)]
    pub pattern: String,

    /// Text or File to match in
    #[arg(short, long)]
    pub input: String,
}