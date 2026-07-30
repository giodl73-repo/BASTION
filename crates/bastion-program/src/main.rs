use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: bastion-program <sem-003..sem-012> <fixture>");
        process::exit(2);
    }
    let input = fs::read_to_string(&args[2]).unwrap_or_else(|error| {
        eprintln!("failed to read fixture: {error}");
        process::exit(2);
    });
    match bastion_program::run(&args[1], &input) {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprintln!("delivery rejected: {error}");
            process::exit(1);
        }
    }
}
