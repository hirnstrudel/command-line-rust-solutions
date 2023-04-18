use clap::Parser;

fn main() {
    if let Err(e) = headr::run(headr::Config::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
