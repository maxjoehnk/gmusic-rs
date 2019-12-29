use std::str::FromStr;
use crate::error::Error;

#[derive(Clone, Debug)]
pub struct GoogleAuth {
    pub token: String
}

impl ToString for GoogleAuth {
    fn to_string(&self) -> String {
        format!("GoogleLogin auth={}", self.token)
    }
}

impl FromStr for GoogleAuth {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<GoogleAuth, Error> {
        Ok(GoogleAuth { token: s.to_owned() })
    }
}

#[cfg(test)]
mod tests {
    use super::GoogleAuth;

    #[test]
    fn test_google_login_auth() {
        let header = GoogleAuth {
            token: "logintoken".to_owned()
        };
        assert_eq!(header.to_string(), "GoogleLogin auth=logintoken".to_string());
    }
}