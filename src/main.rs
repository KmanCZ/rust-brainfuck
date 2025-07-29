use std::{env, process};

enum ProgramError {
    WrongNumberOfArguments,
}

fn check_args(args: &[String]) -> Result<(), ProgramError> {
    if args.len() != 1 {
        return Err(ProgramError::WrongNumberOfArguments);
    }
    Ok(())
}

fn handle_errors(err: ProgramError) {
    match err {
        ProgramError::WrongNumberOfArguments => {
            eprintln!("Error: wrong number of arguments");
        }
    }
    
    process::exit(1);
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(err) = check_args(&args) {
        handle_errors(err);
    }
}
