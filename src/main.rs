use flexi_logger::Duplicate;
use flexi_logger::FileSpec;
use flexi_logger::Logger;
use smw_gfx::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Arguments parse error: {err}");
        process::exit(1);
    });

    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(".log"))
        .duplicate_to_stderr(Duplicate::Warn)
        .duplicate_to_stdout(Duplicate::Info)
        .start()
        .unwrap();

    if let Err(err) = smw_gfx::run(config) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
