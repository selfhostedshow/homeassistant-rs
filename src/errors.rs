use std::fmt;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Config(String),
    Refresh(),
    NoAuth(),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(inner) => write!(f, "{}", inner),
            Error::Config(inner) => write!(f, "{}", inner),
            Error::Refresh() => write!(f, "Tried to refresh a long lived access token"),
            Error::NoAuth() => write!(f, "There are no Authentication Credentals"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Request(inner) => Some(inner),
            Error::Config(_) => None,
            Error::Refresh() => None,
            Error::NoAuth() => None,
        }
    }
}
