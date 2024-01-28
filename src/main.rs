pub mod error;
pub mod object;
pub mod scanner;
pub mod token;
pub mod token_type;

use std::{env::args, fs::File};

fn main() {
    let args = args().collect::<Vec<String>>();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("path should always be valid"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

fn run_prompt() {
    println!("lox-ast");
}

fn run_file(path: &String) -> std::io::Result<()> {
    let f = File::open(path)?;
    let _reader = std::io::BufReader::new(f);
    // todo: convert to byte array
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {}
}
