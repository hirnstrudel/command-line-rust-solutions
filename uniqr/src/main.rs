use clap::Parser;

fn main() {
    if let Err(e) = uniqr::run(uniqr::Config::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
