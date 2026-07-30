use std::env;
use std::fs;
use std::process::ExitCode;

fn usage() {
    eprintln!("usage: bastion assess <safe-synthetic-fixture>");
}

fn run() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 || arguments[0] != "assess" {
        usage();
        return Err("invalid arguments".to_owned());
    }
    let input = fs::read_to_string(&arguments[1])
        .map_err(|error| format!("cannot read fixture: {error}"))?;
    let package = bastion_readiness_slice::parse_package(&input)
        .map_err(|error| format!("invalid fixture: {error}"))?;
    let assessment = bastion_readiness_slice::assess(&package);
    print!("{}", assessment.to_canonical_json());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("bastion: {error}");
            ExitCode::from(2)
        }
    }
}
