fn main() {
    if let Err(error) = agent_runbook::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
