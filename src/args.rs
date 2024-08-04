use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use strum::{Display, EnumString};

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[clap(short, long)]
    pub zone_identifier: String,
    #[clap(short = 'k', long)]
    pub cf_api_key: String,
    #[clap(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
    Import(ImportCmd),
    Delete(DeleteCmd),
}

#[derive(Debug, Parser)]
pub(crate) struct ImportCmd {
    #[clap(short, long)]
    pub export_path: PathBuf,
    #[clap(short, long)]
    pub domain: String,
    #[clap(short = 'a', long)]
    pub destination_address: String,
    #[clap(subcommand)]
    pub export_cmd: ExportSourceCommand,
}

#[derive(Debug, Parser)]
pub(crate) enum ExportSourceCommand {
    #[clap(visible_alias = "sl")]
    SimpleLogin,
    #[clap(visible_alias = "bw")]
    Bitwarden(BitwardenArgs),
}

#[derive(Debug, Clone, Default, EnumString, Display, ValueEnum)]
pub(crate) enum ExportFormat {
    #[strum(serialize = "csv")]
    #[default]
    Csv,
    #[strum(serialize = "json")]
    Json,
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct BitwardenArgs {
    /// The format the export file is in
    #[clap(short, long, default_value_t)]
    pub format: ExportFormat,
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct DeleteCmd {
    #[clap(flatten)]
    pub opts: DeleteOpts,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub(crate) struct DeleteOpts {
    pub route_ids: Vec<String>,
    #[clap(short = 'A', long)]
    pub delete_all: bool,
}
