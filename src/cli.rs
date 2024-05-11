use log::warn; // also available: info, trace.

const HELP: &str = "\
USAGE: sgs [OPTIONS]

OPTIONS:
    -h, --help      Print this help text
    --system FILE   Load the system specified by FILE.
                    FILE must be valid JSON.
                    (Default: Use the bundled system.)
";

#[derive(Debug)]
pub struct AppArgs {
    pub help: bool,
    pub system: Option<String>,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    // If std::env::args() is empty, there's nothing to parse.
    // This happens, e.g., when doing a wasm build.
    if std::env::args().count() == 0 {
        return Ok(AppArgs { help: false, system: None });
    }

    let mut pargs = pico_args::Arguments::from_env();

    let args = AppArgs {
        help: pargs.contains(["-h", "--help"]),
        system: pargs.opt_value_from_str("--system")?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        warn!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

pub fn process_args(args: AppArgs) {
    if args.help {
        println!("{}", HELP);
        std::process::exit(0);
    }
}
