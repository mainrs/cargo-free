use std::{fmt, fmt::Formatter, time::Duration};
use thiserror::Error;

const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

/// The crate's error type.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("Crate name is empty")]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Availability::Available => write!(f, "available"),
            Availability::Unavailable => write!(f, "unavailable"),
            Availability::Unknown => write!(f, "unknown"),
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
    let agent = ureq::builder().timeout(timeout).build();
    let resp = agent.get(&url).call();
    let availability = match resp {
        Ok(resp) => match resp.status() {
            200 => Availability::Unavailable,
            _ => Availability::Unknown,
        },
        Err(e) => match e {
            ureq::Error::Status(code, _) => match code {
                404 => Availability::Available,
                408 => return Err(Error::NetworkTimeout(timeout)),
                _ => Availability::Unknown,
            },
            _ => Availability::Unknown,
        },
    };

    Ok(availability)
}
