mod api;
mod app;

fn main() {
    if let Err(error) = app::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
