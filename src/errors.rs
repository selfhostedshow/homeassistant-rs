use std::fmt;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    HaApi(String),
    Config(String),
    Refresh(),
    NoAuth(),
    PoisonError(
        std::sync::PoisonError<
            std::sync::RwLockReadGuard<'static, std::option::Option<crate::Token>>,
        >,
    ),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}

impl
    From<
        std::sync::PoisonError<
            std::sync::RwLockReadGuard<'static, std::option::Option<crate::Token>>,
        >,
    > for Error
{
    fn from(
        error: std::sync::PoisonError<
            std::sync::RwLockReadGuard<'static, std::option::Option<crate::Token>>,
        >,
    ) -> Self {
        Error::PoisonError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(inner) => write!(f, "{}", inner),
            Error::Config(inner) => write!(f, "{}", inner),
            Error::HaApi(inner) => write!(f, "{}", inner),
            Error::PoisonError(inner) => write!(f, "{}", inner),
            Error::Refresh() => write!(f, "Tried to refresh a long lived access token"),
            Error::NoAuth() => write!(f, "There are no Authentication Credentials"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Request(inner) => Some(inner),
            _ => None,
        }
    }
}
