use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "v0.0.1")]
pub struct Args {
    #[arg(long, help = "Port", default_value_t = 2999)]
    pub port: u16,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// init
    Init,
    /// Search memos (default)
    Search,
    /// Add a new memo
    Add {
        #[arg(long)]
        title: String,

        #[arg(long, default_value = "")]
        description: String,
    },
}
