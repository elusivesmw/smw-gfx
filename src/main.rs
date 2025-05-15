use smw_gfx::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Arguments parse error: {err}");
        process::exit(1);
    });

    if let Err(err) = smw_gfx::run(config) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
