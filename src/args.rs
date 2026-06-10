use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Send files [send /path/to/file]
    Send { file_path: String },
    /// Receive files [receive]
    Receive,
}
