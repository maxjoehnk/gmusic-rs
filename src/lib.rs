use std::fs::{read_to_string, write};

use failure::{ensure, Error};
use oauth2::basic::BasicTokenResponse;
use oauth2::TokenResponse;
use reqwest::{header::AUTHORIZATION, Response};
use serde::Serialize;
use url::Url;

use crate::http::all_tracks::{GetAllTracksRequest, GetAllTracksResponse, Track};
use crate::http::device_management_info::{DeviceManagementInfo, GetDeviceManagementInfoResponse};
use crate::login::perform_oauth;
use crate::http::all_playlists::{GetAllPlaylistsResponse, GetAllPlaylistsRequest, Playlist};

mod http;
mod login;

static BASE_URL: &'static str = "https://mclients.googleapis.com/sj/v2.5/";

#[derive(Debug)]
pub struct GoogleMusicApi {
    auth_token: Option<BasicTokenResponse>,
    device_id: Option<String>,
    client: GoogleMusicApiClient
}

#[derive(Debug)]
pub(crate) struct GoogleMusicApiClient {
    pub id: String,
    pub secret: String
}

impl GoogleMusicApi {
    pub fn new(client_id: String, client_secret: String) -> GoogleMusicApi {
        GoogleMusicApi {
            client: GoogleMusicApiClient {
                id: client_id,
                secret: client_secret
            },
            auth_token: None,
            device_id: None,
        }
    }

    pub async fn login(&mut self) -> Result<(), Error> {
        let token = perform_oauth(&self.client).await?;
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

    pub async fn get_all_tracks(&self) -> Result<Vec<Track>, Error> {
        let body = GetAllTracksRequest::new();
        let res: GetAllTracksResponse = self.json_post(format!("{}trackfeed", BASE_URL).as_str(), &body).await?
            .json()
            .await?;

        Ok(res.data.items)
    }

    pub async fn get_all_playlists(&self) -> Result<Vec<Playlist>, Error> {
        let body = GetAllPlaylistsRequest::new();
        let res: GetAllPlaylistsResponse = self.json_post(format!("{}playlistfeed", BASE_URL).as_str(), &body).await?
            .json()
            .await?;

        Ok(res.data.items)
    }

    pub async fn get_device_management_info(&self) -> Result<Vec<DeviceManagementInfo>, Error> {
        let res: GetDeviceManagementInfoResponse = self.json_get(format!("{}devicemanagementinfo", BASE_URL).as_str())
            .await?
            .json()
            .await?;

        Ok(res.data.items)
    }

    async fn json_post<Request>(&self, url: &str, body: &Request) -> Result<Response, Error>
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
            .send()
            .await?;

        Ok(res)
    }

    async fn json_get(&self, url: &str) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let mut url = Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("dv", "0")
            .append_pair("hl", "en_US")
            .append_pair("tier", "aa");
        let res = client
            .get(url.as_str())
            .header(AUTHORIZATION, self.get_auth_header())
            .send()
            .await?;

        Ok(res)
    }

    fn get_auth_header(&self) -> String {
        let token = self.auth_token.as_ref().unwrap().access_token().secret();
        format!("Bearer {}", token)
    }
}
