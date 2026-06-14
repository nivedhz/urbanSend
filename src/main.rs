mod args;
mod discover;
mod receive;
mod sender;
use crate::args::EntityType;
use args::Arguments;
use clap::Parser;

fn main() {
    let input_args = Arguments::parse();

    match &input_args.entity_type {
        EntityType::Send { file_path } => match sender::send_data(file_path) {
            Ok(_) => println!("[*] File transfer completed successfully!"),
            Err(e) => println!("[!] File transfer failed with error {e}"),
        },
        EntityType::Receive => match receive::receive_data() {
            Ok(_) => println!("[*] Receiver Initialized"),
            Err(e) => println!("[!] Receiver exited with error {e}"),
        },
    };
}
