use clap::{Args, Parser, Subcommand};

pub mod build_lambda;
pub mod create_dart_layer;
pub mod create_dart_project_layer;
pub mod create_flutter_layer;
pub mod fetch_flutter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    BuildLambda(BuildLambdaArgs),
    CreateFlutterLayer(CreateFlutterLayerArgs),
    CreateDartLayer(CreateDartLayerArgs),
    CreateDartProjectLayer(CreateDartProjectLayerArgs),
    FetchFlutter(FetchFlutterArgs),
}

#[derive(Args, Debug)]
pub struct BuildLambdaArgs {
    #[clap(short, long, default_value_t = String::from("aarch64-unknown-linux-gnu"))]
    pub target: String,
    #[clap(short, long, default_value_t = String::from("bootstrap"))]
    pub entrypoint: String,
    #[clap(short, long)]
    pub use_cross: bool,
}

#[derive(Args, Debug)]
pub struct CreateFlutterLayerArgs {}

#[derive(Args, Debug)]
pub struct CreateDartLayerArgs {
    #[clap(short, long)]
    pub version: String,
}

#[derive(Args, Debug)]
pub struct CreateDartProjectLayerArgs {
    #[clap(short, long)]
    pub packages: Vec<String>,
}

#[derive(Args, Debug)]
pub struct FetchFlutterArgs {
    #[clap(short, long, default_value_t = String::from("stable"))]
    pub version: String,
}
