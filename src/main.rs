use std::{env, process, fs::File, io::BufReader};
use std::io::Read;

enum ProgramError {
    WrongNumberOfArguments,
    FileOpenError,
    FileIsDirectory,
    GeneralError,
}

fn check_args(args: &[String]) -> Result<(), ProgramError> {
    if args.len() != 1 {
        return Err(ProgramError::WrongNumberOfArguments);
    }
    Ok(())
}

fn prepare_file(path: &str) -> Result<BufReader<File>, ProgramError> {
    let f = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(ProgramError::FileOpenError),
    };
    
    match f.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Err(ProgramError::FileIsDirectory);
            }
        }
        Err(_) => return Err(ProgramError::GeneralError),
    }

    let reader = BufReader::new(f);

    Ok(reader)
}

fn handle_errors(err: ProgramError) {
    match err {
        ProgramError::WrongNumberOfArguments => {
            eprintln!("Error: wrong number of arguments");
        }
        ProgramError::FileOpenError => {
            eprintln!("Error: failed to open file");
        }
        ProgramError::FileIsDirectory => {
            eprintln!("Error: file is directory");
        }
        ProgramError::GeneralError => {
            eprintln!("Error: something went wrong");
        }
    }

    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(err) = check_args(&args) {
        handle_errors(err);
    }

    let mut f = match prepare_file(&args[0]) {
        Ok(file) => file,
        Err(err) => {
            handle_errors(err);
            return;
        }
    };

    let mut buff = String::new();
    let _ = f.read_to_string(&mut buff);
    println!("{}", buff);
}
