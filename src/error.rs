use reqwest;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    InvalidLogin,
    MissingDeviceId,
    MissingCredentials
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::HttpError(err)
    }
}