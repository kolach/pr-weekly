use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubError {
    #[error("Reqwest error: {source:?}")]
    RequestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("URL parse error: {source:?}")]
    UrlParseError {
        #[from]
        source: url::ParseError,
    },
}
