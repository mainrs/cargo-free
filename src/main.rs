use cargo_free::check_availability;
use indoc::printdoc;
use pico_args::Arguments;
use std::env::args_os;

/// The program's arguments.
struct Args {
    /// True if the help screen should be displayed.
    help: bool,
    /// The crate name to check for availability.
    names: Vec<String>,
    /// True if the tool's version should be displayed.
    version: bool,
}

#[cfg(not(feature = "colors"))]
fn print_help() {
    printdoc! {"
        {crate_name} {crate_version}
        {crate_authors}
        {crate_description}

        USAGE:
            {crate_name} [NAMES]

        FLAGS:
            -h,--help       Prints help information
            -V,--version    Prints version information

        ARGS:
            <NAMES>:         The crate names to check for",
        crate_name = env!("CARGO_PKG_NAME"),
        crate_version = env!("CARGO_PKG_VERSION"),
        crate_authors = env!("CARGO_PKG_AUTHORS"),
        crate_description = env!("CARGO_PKG_DESCRIPTION"),
    };
}

#[cfg(feature = "colors")]
fn print_help() {
    use colored::Colorize;

    printdoc! {"
        {crate_name} {crate_version}
        {crate_authors}
        {crate_description}

        {usage}:
            {crate_name} [NAMES]

        {flags}:
            -h,--help       Prints help information
            -V,--version    Prints version information

        {args}:
            <NAMES>:         The crate names to check for",
        crate_name = env!("CARGO_PKG_NAME"),
        crate_version = env!("CARGO_PKG_VERSION"),
        crate_authors = env!("CARGO_PKG_AUTHORS"),
        crate_description = env!("CARGO_PKG_DESCRIPTION"),
        usage = "USAGE".green(),
        flags = "FLAGS".green(),
        args = "ARGS".green(),
    };
}

#[cfg(not(feature = "colors"))]
fn print_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

#[cfg(feature = "colors")]
fn print_version() {
    use colored::Colorize;
    println!(
        "{} v{}",
        env!("CARGO_PKG_NAME").green(),
        env!("CARGO_PKG_VERSION")
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargo subcommands need to skip the first two arguments as cargo passes the
    // subcommand itself as an argument. The first arg is the binary name
    // (`cargo-free`).
    let args = args_os().skip(2).collect();
    let mut args = Arguments::from_vec(args);
    let args = Args {
        help: args.contains(["-h", "--help"]),
        version: args.contains(["-V", "--version"]),
        names: args.free()?,
    };

    if args.version {
        print_version();
    } else if args.help {
        print_help();
    } else {
        let availabilities = args
            .names
            .into_iter()
            .map(|crate_name| check_availability(crate_name))
            .collect::<Vec<_>>();
        for availability in availabilities {
            match availability {
                Ok(availability) => println!("{}", availability),
                Err(_) => {
                    // TODO: handle as well. Maybe print name.
                }
            }
        }
    }

    Ok(())
}
