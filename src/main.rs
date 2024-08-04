use clap::Parser;

mod args;
mod error;
mod handlers;
mod models;

use args::{Args, Subcommand};
use error::Error;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let result = match &args.cmd {
        Subcommand::Import(cmd) => handlers::import::handle(&args, cmd).await,
        Subcommand::Delete(cmd) => handlers::delete::handle(&args, cmd).await,
    };
}
