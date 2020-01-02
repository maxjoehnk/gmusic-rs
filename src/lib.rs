use std::fs::{read_to_string, write};

use failure::{ensure, format_err, Error};
use hmac::{Hmac, Mac};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use reqwest::{header::AUTHORIZATION, Response};
use sha1::Sha1;
use url::Url;

use crate::http::all_playlists::{GetAllPlaylistsRequest, GetAllPlaylistsResponse};
pub use crate::http::all_playlists::{Playlist, PlaylistShareState, PlaylistType};
pub use crate::http::all_tracks::Track;
use crate::http::all_tracks::{GetAllTracksRequest, GetAllTracksResponse};
use crate::http::device_management_info::{DeviceManagementInfo, GetDeviceManagementInfoResponse};
use crate::http::playlist_entries::GetPlaylistEntriesResponse;
pub use crate::http::playlist_entries::PlaylistEntry;
use crate::login::perform_oauth;
use crate::token::AuthToken;
use std::time::{SystemTime, UNIX_EPOCH};

mod http;
mod login;
mod token;

static BASE_URL: &str = "https://mclients.googleapis.com/sj/v2.5/";
static STREAM_URL: &str = "https://mclients.googleapis.com/music/mplay";
static REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

#[derive(Debug, Clone)]
pub struct GoogleMusicApi {
    auth_token: AuthToken,
    device_id: Option<String>,
    client: GoogleMusicApiClient,
}

#[derive(Debug, Clone)]
pub(crate) struct GoogleMusicApiClient {
    pub id: String,
    pub secret: String,
    oauth_client: BasicClient,
}

impl GoogleMusicApi {
    pub fn new(client_id: String, client_secret: String) -> Result<GoogleMusicApi, Error> {
        let oauth_client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
            Some(TokenUrl::new(
                "https://www.googleapis.com/oauth2/v3/token".to_string(),
            )?),
        )
        .set_redirect_url(RedirectUrl::new(REDIRECT_URI.to_string())?);

        Ok(GoogleMusicApi {
            client: GoogleMusicApiClient {
                id: client_id,
                secret: client_secret,
                oauth_client,
            },
            auth_token: AuthToken::new(),
            device_id: None,
        })
    }

    pub fn login(&self) -> Result<(), Error> {
        let token = perform_oauth(&self.client.oauth_client)?;
        self.auth_token.set_token(token);
        Ok(())
    }

    pub fn store_token(&self) -> Result<(), Error> {
        ensure!(self.auth_token.has_token(), "No token available to persist");
        let token = serde_json::to_string(&self.auth_token.get_token()?)?;
        write(".google-auth.json", token)?;
        Ok(())
    }

    pub fn load_token(&mut self) -> Result<(), Error> {
        let token = read_to_string(".google-auth.json")?;
        let token = serde_json::from_str(&token)?;
        self.auth_token.set_token(token);
        Ok(())
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>, Error> {
        let body = GetAllTracksRequest::new();
        let res: GetAllTracksResponse = self
            .json_post(format!("{}trackfeed", BASE_URL).as_str(), &body)?
            .json()?;

        Ok(res.data.items)
    }

    pub fn get_all_playlists(&self) -> Result<Vec<Playlist>, Error> {
        let body = GetAllPlaylistsRequest::new();
        let res: GetAllPlaylistsResponse = self
            .json_post(format!("{}playlistfeed", BASE_URL).as_str(), &body)?
            .json()?;

        Ok(res.data.items)
    }

    pub fn get_device_management_info(&self) -> Result<Vec<DeviceManagementInfo>, Error> {
        let res: GetDeviceManagementInfoResponse = self
            .json_get(format!("{}devicemanagementinfo", BASE_URL).as_str())?
            .json()?;

        Ok(res.data.items)
    }

    pub fn get_playlist_entries(&self) -> Result<Vec<PlaylistEntry>, Error> {
        let url = format!("{}plentryfeed", BASE_URL);
        let mut res: GetPlaylistEntriesResponse = self.json_post(&url, &())?.json()?;

        for entry in &mut res.data.items {
            if let Some(mut track) = entry.track.as_mut() {
                track.id = entry.track_id.clone()
            }
        }

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
            .header(AUTHORIZATION, self.auth_token.get_auth_header()?)
            .header("X-Device-ID", device_id)
            .send()?;

        Ok(res.url().clone())
    }

    fn get_signature(id: &str) -> Result<(String, String), Error> {
        let key_1 = base64::decode("VzeC4H4h+T2f0VI180nVX8x+Mb5HiTtGnKgH52Otj8ZCGDz9jRWyHb6QXK0JskSiOgzQfwTY5xgLLSdUSreaLVMsVVWfxfa8Rw==")?;
        let key_2 = base64::decode("ZAPnhUkYwQ6y5DdQxWThbvhJHN8msQ1rqJw0ggKdufQjelrKuiGGJI30aswkgCWTDyHkTGK9ynlqTkJ5L4CiGGUabGeo8M6JTQ==")?;

        let key: Vec<u8> = key_1.iter().zip(key_2.iter()).map(|(a, b)| a ^ b).collect();

        let salt = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64() * 1000f64;
        let salt = salt.floor();
        let salt = format!("{}", salt);

        let mut mac = Hmac::<Sha1>::new_varkey(&key)
            .map_err(|err| format_err!("Invalid key length {:?}", err))?;
        mac.input(id.as_bytes());
        mac.input(salt.as_bytes());

        let signature = base64::encode(&mac.result().code());

        Ok((signature, salt.to_string()))
    }

    fn json_post<Request>(&self, url: &str, body: &Request) -> Result<Response, Error>
    where
        Request: serde::Serialize,
    {
        if self.auth_token.requires_new_token() {
            self.auth_token.refresh(&self.client.oauth_client)?;
        }
        let client = reqwest::Client::new();
        let mut url = Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("dv", "0")
            .append_pair("hl", "en_US")
            .append_pair("tier", "aa");
        let res = client
            .post(url.as_str())
            .json(body)
            .header(AUTHORIZATION, self.auth_token.get_auth_header()?)
            .send()?
            .error_for_status()?;

        Ok(res)
    }

    fn json_get(&self, url: &str) -> Result<Response, Error> {
        if self.auth_token.requires_new_token() {
            self.auth_token.refresh(&self.client.oauth_client)?;
        }
        let client = reqwest::Client::new();
        let mut url = Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("dv", "0")
            .append_pair("hl", "en_US")
            .append_pair("tier", "aa");
        let res = client
            .get(url.as_str())
            .header(AUTHORIZATION, self.auth_token.get_auth_header()?)
            .send()?
            .error_for_status()?;

        Ok(res)
    }
}
