use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "compare" {
        eprintln!("usage: bastion-remedies compare <fixture>");
        process::exit(2);
    }
    let input = fs::read_to_string(&args[2]).unwrap_or_else(|error| {
        eprintln!("failed to read fixture: {error}");
        process::exit(2);
    });
    match bastion_remedy_slice::run(&input) {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprintln!("comparison rejected: {error}");
            process::exit(1);
        }
    }
}
