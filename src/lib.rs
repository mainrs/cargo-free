#[cfg(feature = "colors")]
use colored::Colorize;
use std::{fmt, fmt::Formatter, time::Duration};
use thiserror::Error;

const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("crate name is empty")]
    EmptyCrateName,
    #[error("API request to crates.io timed out after {0:?}")]
    NetworkTimeout(Duration),
}

/// The availability status of a crate name.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Availability {
    /// The crate name is available.
    Available,
    /// The crate name is unavailable.
    Unavailable,
    /// The crate name can't be resolved.
    Unknown,
}

impl fmt::Display for Availability {
    #[cfg(not(feature = "colors"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "Available"),
            Availability::Unavailable => write!(f, "Unavailable"),
            Availability::Unknown => write!(f, "Unknown"),
        }
    }

    #[cfg(feature = "colors")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "{}", "Available".green()),
            Availability::Unavailable => write!(f, "{}", "Unavailable".red()),
            Availability::Unknown => write!(f, "{}", "Unknown".bright_black()),
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
/// `Ok(Availability)` if the name could be resolved. If the crate name is
/// empty, `Err(Error::EmptyCrateName)` gets returned. Returns
/// `Err(Error::NetworkTimeout)` if a timeout occurred.
///
/// # Note
///
/// The needed network request will timeout after five seconds.
pub fn check_availability(name: impl AsRef<str>) -> Result<Availability, Error> {
    check_availability_with_timeout(name, Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
}

/// Checks the availability for a given crate name. Stops after the given
/// timeout duration and returns `Availability::Unknown`.
///
/// # Arguments
///
/// - `name`: The crate name to check
/// - `timeout`: The timeout after which to stop trying to connect to the
///   crates.io API.
///
/// # Returns
///
/// `Ok(Availability)` if the name could be resolved. If the crate name is
/// empty, `Err(Error::EmptyCrateName)` gets returned. Returns
/// `Err(Error::NetworkTimeout)` if a timeout occurred.
pub fn check_availability_with_timeout(
    name: impl AsRef<str>,
    timeout: Duration,
) -> Result<Availability, Error> {
    let name = name.as_ref();
    if name.is_empty() {
        return Err(Error::EmptyCrateName);
    }

    let url = format!("https://crates.io/api/v1/crates/{}", name);
    let resp = ureq::get(&url).timeout(timeout).call();
    let availability = match resp.status() {
        200 => Availability::Unavailable,
        404 => Availability::Available,
        408 => return Err(Error::NetworkTimeout(timeout)),
        _ => Availability::Unknown,
    };
    Ok(availability)
}
