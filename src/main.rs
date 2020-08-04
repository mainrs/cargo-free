use cargo_free::check_availability;
use indoc::printdoc;
use pico_args::Arguments;

/// The program's arguments.
struct Args {
    /// True if the help screen should be displayed.
    help: bool,
    /// The crate name to check for availability.
    names: Vec<String>,
    /// True if the tool's version should be displayed.
    version: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    println!("{:?}", &args);
    let args = Args {
        help: args.contains(["-h", "--help"]),
        version: args.contains(["-V", "--version"]),
        names: args.free()?,
    };

    if args.version {
        println!("cargo-free v{}", env!("CARGO_PKG_VERSION"));
    } else if args.help {
        printdoc! {"
            {crate_name} {crate_version}
            {crate_authors}
            {crate_description}

            USAGE:
                {crate_name} [NAME]

            FLAGS:
                -h,--help       Prints help information
                -V,--version    Prints version information

            ARGS:
                <NAME>:         The crate name to check for",
            crate_name = env!("CARGO_PKG_NAME"),
            crate_version = env!("CARGO_PKG_VERSION"),
            crate_authors = env!("CARGO_PKG_AUTHORS"),
            crate_description = env!("CARGO_PKG_DESCRIPTION"),
        };
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
