use std::process::ExitCode;

fn main() -> ExitCode {
    match tas_showcase::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
