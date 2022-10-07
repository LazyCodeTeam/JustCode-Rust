use clap::Parser;
use command::{
    build_lambda::build_lambda, create_dart_layer::create_dart_layer,
    create_dart_project_layer::create_dart_project_layer,
    create_flutter_layer::create_flutter_layer, fetch_flutter::fetch_flutter, Cli, Command,
};
use model::error::DynError;

mod command;
mod model;
mod util;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let cli = Cli::parse();

    match &cli.command {
        Command::BuildLambda(args) => build_lambda(args),
        Command::CreateFlutterLayer(args) => create_flutter_layer(args),
        Command::CreateDartLayer(args) => create_dart_layer(args),
        Command::CreateDartProjectLayer(args) => create_dart_project_layer(args),
        Command::FetchFlutter(args) => fetch_flutter(args),
    }
}
