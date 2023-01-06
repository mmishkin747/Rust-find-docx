fn main() {
    if let Err(e) = rfd::get_args().and_then(rfd::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
