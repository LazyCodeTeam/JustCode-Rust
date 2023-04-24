use clap::Parser;
use command::{build_lambdas::build_lambdas, Cli, Command};

mod command;
mod util;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::BuildLambdas(args) => build_lambdas(args),
    }
}
