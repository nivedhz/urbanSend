mod args;
mod discover;
mod receive;
mod sender;
use crate::args::EntityType;
use anyhow::Result;
use args::Arguments;
use clap::Parser;

fn main() -> Result<()> {
    let input_args = Arguments::parse();

    match &input_args.entity_type {
        EntityType::Send { file_path } => sender::send_data(String::from(file_path))?,
        EntityType::Receive => receive::receive_data()?,
    };

    Ok(())
}
