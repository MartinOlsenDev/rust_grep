use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Pattern to match on
    #[arg(short, long)]
    pattern: String,

    /// Text or File to match in
    #[arg(short, long)]
    input: String,
}