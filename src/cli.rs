use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[arg(long, default_value = "cli")]
    pub mode: Mode,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        content: String,
    },
    List,
    Update {
        #[arg(short, long)]
        id: i64,
        #[arg(short, long)]
        content: String,
    },
    Remove {
        #[arg(short, long)]
        id: i64,
    },
}
#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
    Web,
    Cli,
}