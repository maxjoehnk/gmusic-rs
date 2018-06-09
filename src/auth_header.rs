use reqwest::header;
use std::fmt;
use std::str::FromStr;
use error::Error;

#[derive(Clone, Debug)]
pub struct GoogleAuth {
    pub token: String
}

impl header::Scheme for GoogleAuth {
    fn scheme() -> Option<&'static str> {
        Some("GoogleLogin")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "auth={}", self.token)
    }
}

impl FromStr for GoogleAuth {
    type Err = ::Error;
    fn from_str(s: &str) -> Result<GoogleAuth, Error> {
        Ok(GoogleAuth { token: s.to_owned() })
    }
}

#[cfg(test)]
mod tests {
    use reqwest::header::{Authorization, Headers};
    use super::GoogleAuth;

    #[test]
    fn test_google_login_auth() {
        let mut headers = Headers::new();
        headers.set(Authorization(GoogleAuth {
            token: "logintoken".to_owned()
        }));
        assert_eq!(headers.to_string(), "Authorization: GoogleLogin auth=logintoken\r\n".to_owned());
    }
}