mod lib;

fn main() {
    if let Err(error) = lib::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}