extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate serde_ini;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate env_logger;
extern crate log;
extern crate url;
extern crate rand;
extern crate base64;
extern crate sha1;
extern crate byteorder;
extern crate openssl;

mod auth;
mod auth_header;
mod error;
mod http;

use auth_header::GoogleAuth;
use error::Error;
use http::login::{LoginResponse, LoginRequest};
use http::oauth::{OAuthRequest, OAuthResponse};
use http::settings::{GetSettingsRequest, GetSettingsResponse, Settings};
use http::get_all_tracks::{GetAllTracksRequest, GetAllTracksResponse, Track};

static BASE_URL: &'static str = "https://www.googleapis.com/sj/v1.11/";
static WEB_URL: &'static str = "https://play.google.com/music/";
static MOBILE_URL: &'static str = "https://android.clients.google.com/music/";
static ACCOUNT_URL: &'static str = "https://www.google.com/accounts/";
static AUTH_URL: &'static str = "https://android.clients.google.com/auth";

header! {
    (XDeviceId, "X-Device-ID") => [String]
}

#[derive(Debug)]
pub struct GoogleMusicAPI {
    email: Option<String>,
    password: Option<String>,
    android_id: String,
    master_token: Option<String>,
    auth_token: Option<String>,
    device_id: Option<String>
}

impl GoogleMusicAPI {
    pub fn new(email: String, password: String, android_id: Option<String>) -> GoogleMusicAPI {
        let android_id = android_id.unwrap_or(auth::create_android_id());
        GoogleMusicAPI {
            email: Some(email),
            password: Some(password),
            android_id,
            master_token: None,
            auth_token: None,
            device_id: None
        }
    }

    pub fn new_with_token(token: String, android_id: Option<String>) -> GoogleMusicAPI {
        let android_id = android_id.unwrap_or(auth::create_android_id());
        GoogleMusicAPI {
            email: None,
            password: None,
            android_id,
            master_token: Some(token),
            auth_token: None,
            device_id: None
        }
    }

    pub fn login(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let req = LoginRequest::new(self.email.clone().unwrap(), self.password.clone().unwrap(), self.android_id.clone());
        let req = serde_urlencoded::to_string(req).unwrap();
        let mut res = client
            .post(AUTH_URL)
            .header(reqwest::header::ContentType::form_url_encoded())
            .body(req)
            .send()?;
        if res.status().is_success() {
            let body = res.text()?;
            let res: LoginResponse = serde_ini::from_str(body.as_str()).unwrap();
            self.master_token = Some(res.token);
            return Ok(());
        }
        Err(Error::InvalidLogin)
    }

    pub fn init(&mut self) -> Result<(), Error> {
        let key_1 = base64::decode("VzeC4H4h+T2f0VI180nVX8x+Mb5HiTtGnKgH52Otj8ZCGDz9jRWyHb6QXK0JskSiOgzQfwTY5xgLLSdUSreaLVMsVVWfxfa8Rw==").unwrap();
        let key_2 = base64::decode("ZAPnhUkYwQ6y5DdQxWThbvhJHN8msQ1rqJw0ggKdufQjelrKuiGGJI30aswkgCWTDyHkTGK9ynlqTkJ5L4CiGGUabGeo8M6JTQ==").unwrap();

        let key: Vec<u8> = key_1
            .iter()
            .zip(key_2.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        let android_id = self.android_id.clone();

        let req = if self.master_token.is_some() {
            let master_token = self.master_token.clone().unwrap();
            Some(OAuthRequest::from_token(android_id, master_token))
        }else if self.email.is_some() && self.password.is_some() {
            let email = self.email.clone().unwrap();
            let password = self.password.clone().unwrap();
            Some(OAuthRequest::from_userdata(android_id, email, password))
        }else {
            None
        };

        match req {
            Some(req) => {
                let client = reqwest::Client::new();
                let req = serde_urlencoded::to_string(req).unwrap();
                let mut res = client
                    .post(AUTH_URL)
                    .header(reqwest::header::ContentType::form_url_encoded())
                    .body(req)
                    .send()?;
                if res.status().is_success() {
                    let body = res.text()?;
                    let res: OAuthResponse = serde_ini::from_str(body.as_str()).unwrap();
                    self.auth_token = Some(res.auth);
                    return Ok(());
                }
                Err(Error::InvalidLogin)
            },
            None => Err(Error::MissingCredentials)
        }
    }

    pub fn get_settings(&self) -> Result<Settings, Error> {
        let client = reqwest::Client::new();
        let req = GetSettingsRequest::new();
        let req = serde_json::to_string(&req).unwrap();
        let mut res = client
            .post(format!("{}services/fetchsettings?u=0", WEB_URL).as_str())
            .header(reqwest::header::ContentType::json())
            .header(reqwest::header::Authorization(GoogleAuth {
                token: self.master_token.clone().unwrap()
            }))
            .body(req)
            .send()?;
        if res.status().is_success() {
            let body = res.text()?;
            let res: GetSettingsResponse = serde_json::from_str(body.as_str()).unwrap();
            return Ok(res.response.settings);
        }
        Err(Error::InvalidLogin)
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>, Error> {
        let client = reqwest::Client::new();
        let body = GetAllTracksRequest::new(1000);
        let body = serde_json::to_string(&body).unwrap();
        let h = reqwest::header::Authorization(GoogleAuth {
            token: self.auth_token.clone().unwrap()
        });
        println!("{:?}", h);
        let mut res = client
            .post(format!("{}trackfeed", BASE_URL).as_str())
            .header(reqwest::header::ContentType::json())
            .header(reqwest::header::Authorization(GoogleAuth {
                token: self.master_token.clone().unwrap()
            }))
            .body(body)
            .send()?;
        println!("{:?}", res);
        if res.status().is_success() {
            let body = res.text()?;
            println!("{:?}", body);
            let res: GetAllTracksResponse = serde_json::from_str(&body).unwrap();
            return Ok(res.data.items);
        }
        Err(Error::InvalidLogin)
    }

    pub fn get_stream_url(&self, id: String) -> Result<(), Error> {
        if self.device_id.is_none() {
            return Err(Error::MissingDeviceId);
        }

        let query = id;

        let client = reqwest::Client::new();
        let mut res = client
            .get(format!("{}mplay?={}", MOBILE_URL, query).as_str())
            .header(XDeviceId(self.device_id.clone().unwrap()))
            .send()?;

        if res.status().is_success() {
            let body = res.text()?;
            println!("{:?}", body);
            return Ok(());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        let email = env!("GMUSIC_EMAIL").to_owned();
        let password = env!("GMUSIC_PASSWORD").to_owned();
        let mut client = GoogleMusicAPI::new(email.clone(), password.clone(), None);
        assert!(client.login().is_ok());
    }

    #[test]
    fn test_init_with_userdata() {
        let email = env!("GMUSIC_EMAIL").to_owned();
        let password = env!("GMUSIC_PASSWORD").to_owned();
        let mut client = GoogleMusicAPI::new(email.clone(), password.clone(), None);
        assert!(client.init().is_ok());
    }

    #[test]
    fn test_init_with_token() {
        let master_token = env!("GMUSIC_MASTER_TOKEN").to_owned();
        let mut client = GoogleMusicAPI::new_with_token(master_token, None);
        assert!(client.init().is_ok());
    }

    fn create_initialized_client() -> GoogleMusicAPI {
        let email = env!("GMUSIC_EMAIL").to_owned();
        let password = env!("GMUSIC_PASSWORD").to_owned();
        let mut client = GoogleMusicAPI::new(email.clone(), password.clone(), None);
        client.init().unwrap();
        client
    }

    #[test]
    fn test_get_all_tracks() {
        let client = create_initialized_client();
        let tracks = client.get_all_tracks().unwrap();
        assert!(tracks.len() > 0);
    }
}