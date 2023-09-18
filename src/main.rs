use std::env;
use std::process;
use smw_gfx::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Arguments parse error: {err}");
        process::exit(1);
    });

    if let Err(err) = smw_gfx::run(config) {
        println!("Error: {err}");
        process::exit(1);
    }
}
