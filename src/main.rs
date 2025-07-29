use std::env;
#[derive(Debug)]
enum ProgramError {
    WrongNumberOfArguments,
}

fn check_args(args: &[String]) -> Result<(), ProgramError> {
    if args.len() != 1 {
        return Err(ProgramError::WrongNumberOfArguments);
    }
    Ok(())
}
fn main() -> Result<(), ProgramError> {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(err) = check_args(&args) {
        return Err(err);
    }
    
    Ok(())
}
