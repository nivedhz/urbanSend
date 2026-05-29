mod discover;
mod receive;
mod sender;
use anyhow::Result;
use std::env::args;

fn main() -> Result<()> {
    let input_args: Vec<String> = args().collect();
    if input_args.len() > 1 {
        let operation = &input_args[1];

        match operation.trim() {
            "send" => {
                let file_path = String::from(&input_args[2]);
                sender::send_data(file_path)?
            }
            "receive" => receive::receive_data()?,
            _ => eprintln!("Invalid Operation\n"),
        }
    }

    Ok(())
}
