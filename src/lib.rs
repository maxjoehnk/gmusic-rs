use std::fs::{read_to_string, write};
use std::time::{SystemTime, UNIX_EPOCH};

use failure::{ensure, Error, format_err};
use hmac::{Hmac, Mac};
use oauth2::basic::BasicTokenResponse;
use oauth2::TokenResponse;
use reqwest::{header::AUTHORIZATION, Response};
use serde::Serialize;
use sha1::Sha1;
use url::Url;

use crate::http::all_playlists::{GetAllPlaylistsRequest, GetAllPlaylistsResponse};
pub use crate::http::all_playlists::{AlbumArtRef, Playlist, PlaylistShareState, PlaylistType};
use crate::http::all_tracks::{GetAllTracksRequest, GetAllTracksResponse};
pub use crate::http::all_tracks::Track;
use crate::http::device_management_info::{DeviceManagementInfo, GetDeviceManagementInfoResponse};
use crate::login::perform_oauth;

mod http;
mod login;

static BASE_URL: &'static str = "https://mclients.googleapis.com/sj/v2.5/";
static STREAM_URL: &'static str = "https://mclients.googleapis.com/music/mplay";

#[derive(Debug, Clone)]
pub struct GoogleMusicApi {
    auth_token: Option<BasicTokenResponse>,
    device_id: Option<String>,
    client: GoogleMusicApiClient,
}

#[derive(Debug, Clone)]
pub(crate) struct GoogleMusicApiClient {
    pub id: String,
    pub secret: String,
}

impl GoogleMusicApi {
    pub fn new(client_id: String, client_secret: String) -> GoogleMusicApi {
        GoogleMusicApi {
            client: GoogleMusicApiClient {
                id: client_id,
                secret: client_secret,
            },
            auth_token: None,
            device_id: None,
        }
    }

    pub fn login(&mut self) -> Result<(), Error> {
        let token = perform_oauth(&self.client)?;
        self.auth_token = Some(token);
        Ok(())
    }

    pub fn store_token(&self) -> Result<(), Error> {
        ensure!(self.auth_token.is_some(), "No token available to persist");
        let token = self.auth_token.as_ref().unwrap();
        let token = serde_json::to_string(token)?;
        write(".google-auth.json", token)?;
        Ok(())
    }

    pub fn load_token(&mut self) -> Result<(), Error> {
        let token = read_to_string(".google-auth.json")?;
        let token = serde_json::from_str(&token)?;
        self.auth_token = Some(token);
        Ok(())
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>, Error> {
        let body = GetAllTracksRequest::new();
        let res: GetAllTracksResponse = self.json_post(format!("{}trackfeed", BASE_URL).as_str(), &body)?.json()?;

        Ok(res.data.items)
    }

    pub fn get_all_playlists(&self) -> Result<Vec<Playlist>, Error> {
        let body = GetAllPlaylistsRequest::new();
        let res: GetAllPlaylistsResponse = self.json_post(format!("{}playlistfeed", BASE_URL).as_str(), &body)?.json()?;

        Ok(res.data.items)
    }

    pub fn get_device_management_info(&self) -> Result<Vec<DeviceManagementInfo>, Error> {
        let res: GetDeviceManagementInfoResponse = self.json_get(format!("{}devicemanagementinfo", BASE_URL).as_str())?.json()?;

        Ok(res.data.items)
    }

    pub fn get_stream_url(&self, id: &str, device_id: &str) -> Result<Url, Error> {
        let client = reqwest::Client::new();
        let mut url = Url::parse(STREAM_URL)?;
        let (sig, salt) = GoogleMusicApi::get_signature(id)?;
        url.query_pairs_mut()
            .append_pair("dv", "0")
            .append_pair("hl", "en_US")
            .append_pair("tier", "aa")
            .append_pair("opt", "hi")
            .append_pair("net", "mob")
            .append_pair("pt", "e")
            .append_pair("slt", &salt)
            .append_pair("sig", &sig)
            .append_pair("songid", id);
        let res = client
            .get(url.as_str())
            .header(AUTHORIZATION, self.get_auth_header())
            .header("X-Device-ID", device_id)
            .send()?;

        Ok(res.url().clone())
    }

    fn get_signature(id: &str) -> Result<(String, String), Error> {
        let key_1 = base64::decode("VzeC4H4h+T2f0VI180nVX8x+Mb5HiTtGnKgH52Otj8ZCGDz9jRWyHb6QXK0JskSiOgzQfwTY5xgLLSdUSreaLVMsVVWfxfa8Rw==")?;
        let key_2 = base64::decode("ZAPnhUkYwQ6y5DdQxWThbvhJHN8msQ1rqJw0ggKdufQjelrKuiGGJI30aswkgCWTDyHkTGK9ynlqTkJ5L4CiGGUabGeo8M6JTQ==")?;

        let key: Vec<u8> = key_1
            .iter()
            .zip(key_2.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        let salt = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64() * 1000f64;
        let salt = salt.floor();
        let salt = format!("{}", salt);

        let mut mac = Hmac::<Sha1>::new_varkey(&key).map_err(|err| format_err!("Invalid key length {:?}", err))?;
        mac.input(id.as_bytes());
        mac.input(salt.as_bytes());

        let signature = base64::encode(&mac.result().code());

        Ok((signature, salt.to_string()))
    }

    fn json_post<Request>(&self, url: &str, body: &Request) -> Result<Response, Error>
        where Request: Serialize {
        let client = reqwest::Client::new();
        let mut url = Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("dv", "0")
            .append_pair("hl", "en_US")
            .append_pair("tier", "aa");
        let res = client
            .post(url.as_str())
            .json(body)
            .header(AUTHORIZATION, self.get_auth_header())
            .send()?;

        Ok(res)
    }

    fn json_get(&self, url: &str) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let mut url = Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("dv", "0")
            .append_pair("hl", "en_US")
            .append_pair("tier", "aa");
        let res = client
            .get(url.as_str())
            .header(AUTHORIZATION, self.get_auth_header())
            .send()?;

        Ok(res)
    }

    fn get_auth_header(&self) -> String {
        let token = self.auth_token.as_ref().unwrap().access_token().secret();
        format!("Bearer {}", token)
    }
}
