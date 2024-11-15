use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occurred while fetching: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// An error occurred while parsing JSON.
    #[error("An error occurred while parsing JSON: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("An error occurred while processing regex: {0}")]
    FancyRegex(#[from] fancy_regex::Error),

    #[error("An error occurred while initializing ramdl: {0}")]
    Init(String),
}
