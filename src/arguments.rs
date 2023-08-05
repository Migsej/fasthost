use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramArgs {
    /// file to host
    pub file: String,

    /// port to host on
    #[arg(short, long, default_value = "12345")]
    pub port: u16,
}


