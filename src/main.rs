use cargo_free::{check_availability, Availability, Error};
use clap::{AppSettings, Clap};
use serde_json::json;
use std::process::exit;
use terminal_log_symbols::colored::{ERROR_SYMBOL, SUCCESS_SYMBOL, UNKNOWN_SYMBOL};
use terminal_spinners::{SpinnerBuilder, DOTS};

/// XXX: There is no first-class support for cargo subcommands. This is
/// basically the "official" workaround.
#[derive(Clap, Debug)]
#[clap(
    author,
    bin_name("cargo-free"),
    setting(AppSettings::ColoredHelp),
    version
)]
enum Cli {
    #[clap(
        name = "free",
        setting(AppSettings::DeriveDisplayOrder),
        setting(AppSettings::UnifiedHelpMessage)
    )]
    Free(FreeArgs),
}

#[derive(Clap, Debug)]
struct FreeArgs {
    /// Output result as json object.
    #[clap(long, short)]
    json: bool,

    /// The crate name to check for availability.
    names: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli {
        Cli::Free(args) => {
            // The spinner should only be shown if the user does not want json, as the
            // spinner will interfere with piping otherwise.
            let mut handle = None;
            if !args.json {
                handle = Some(
                    SpinnerBuilder::new()
                        .spinner(&DOTS)
                        .text("Fetching metadata from crates.io ...")
                        .start(),
                );
            }

            // Calculate the maximum character length of all crate names supplied alongside
            // their availability.
            let mut max_length_crate_name = 0;
            let availabilities = args
                .names
                .iter()
                .map(|crate_name| {
                    let crate_name_length = crate_name.len();
                    if crate_name_length > max_length_crate_name {
                        max_length_crate_name = crate_name_length;
                    }

                    (crate_name, check_availability(&crate_name))
                })
                .collect::<Vec<_>>();
            // Check if the list is empty (user did not supply any crate names).
            if availabilities.is_empty() {
                if let Some(handle) = handle {
                    handle.text("No crate names supplied!");
                    handle.error();
                } else {
                    eprintln!("No crate names supplied!");
                }
                exit(1);
            }

            if let Some(handle) = handle {
                handle.stop_and_clear();
            }

            if args.json {
                let mut objects = Vec::with_capacity(availabilities.len());
                for (crate_name, available) in availabilities {
                    if let Ok(available) = available {
                        objects.push(json!({
                            "crate": crate_name,
                            "availability": available.to_string(),
                        }));
                    }
                }

                println!("{}", json!(objects));
            } else {
                print(availabilities);
            }
        }
    }

    Ok(())
}

fn print(availabilities: Vec<(&String, Result<Availability, Error>)>) {
    for (crate_name, available) in availabilities {
        if let Ok(available) = available {
            let emoji = match available {
                Availability::Available => SUCCESS_SYMBOL,
                Availability::Unavailable => ERROR_SYMBOL,
                Availability::Unknown => UNKNOWN_SYMBOL,
            };
            println!("{} {}", emoji, crate_name);
        }
    }
}
