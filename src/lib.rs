use colored::Colorize;
use std::{fmt, fmt::Formatter, time::Duration};

const TIMEOUT: u64 = 5;

/// The availability status of a crate name.
pub enum Availability {
    /// The crate name is available.
    Available,
    /// The crate name is unavailable.
    Unavailable,
    /// The crate name can't be resolved.
    Unknown,
}

impl fmt::Display for Availability {
    #[cfg(not(feature = "color"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "Available"),
            Availability::Unavailable => write!(f, "Unavailable"),
            Availability::Unknown => write!(f, "Unknown"),
        }
    }

    #[cfg(feature = "color")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "Available".green()),
            Availability::Unavailable => write!(f, "Unavailable".red()),
            Availability::Unknown => write!(f, "Unknown".bright_black()),
        }
    }
}

/// Checks the availability for a given crate name.
///
/// # Arguments
///
/// - `name`: The crate name to check
///
/// # Returns
///
/// `Ok(Availability)` if the name could be resolved, an error otherwise.
///
/// # Note
///
/// The needed network request will timeout after five seconds.
pub fn check_availability(name: impl AsRef<str>) -> Result<Availability, ()> {
    let name = name.as_ref();
    if name.is_empty() {
        eprintln!("Crate name can't be empty");
        return Err(());
    }

    let url = format!("https://crates.io/api/v1/crates/{}", name);
    let resp = ureq::get(&url).timeout(Duration::from_secs(TIMEOUT)).call();
    let availability = match resp.status() {
        200 => Availability::Unavailable,
        404 => Availability::Available,
        _ => Availability::Unknown,
    };
    Ok(availability)
}
