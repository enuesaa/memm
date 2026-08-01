use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version = "v0.0.1", disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(long, help = "Port", default_value_t = 2999)]
    port: u16,
    
    #[arg(long, action = ArgAction::Help, help = "Print help")]
    help: bool,

    #[arg(long, action = ArgAction::Version, help = "Print version")]
    version: bool,
}

fn main() {
    let args = Args::parse();

    println!("{args:?}");
}
