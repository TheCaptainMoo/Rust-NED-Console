use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliOutput {
    /// File to input
    #[arg(short, long)]
    pub input_file: String,
    /// Encrypt or decrypt
    #[arg(short, long)]
    pub decider: String,
    /// Default or ASCII
    #[arg(short, long)]
    pub mode: String,
    /// Key
    #[arg(short, long)]
    pub key: String,
    /// Recursion
    #[arg(short, long)]
    pub recursion: String,
}