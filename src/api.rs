use std::fs::{read_to_string, write};
use std::time::{SystemTime, UNIX_EPOCH};

use failure::{ensure, format_err, Error};
use hmac::{Hmac, Mac};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl, TokenUrl};
use reqwest::header::HeaderMap;
use reqwest::{header::AUTHORIZATION, Method, RequestBuilder, Response, StatusCode};
use sha1::Sha1;
use url::Url;

use crate::auth::{get_oauth_url, perform_oauth, request_token};
use crate::models::album::Album;
use crate::models::all_playlists::Playlist;
use crate::models::all_playlists::{GetAllPlaylistsRequest, GetAllPlaylistsResponse};
use crate::models::all_tracks::Track;
use crate::models::all_tracks::{GetAllTracksRequest, GetAllTracksResponse};
use crate::models::artist::Artist;
use crate::models::device_management_info::{
    DeviceManagementInfo, GetDeviceManagementInfoResponse,
};
use crate::models::playlist_entries::PlaylistEntry;
use crate::models::playlist_entries::{GetPlaylistEntriesRequest, GetPlaylistEntriesResponse};
use crate::models::search_results::{SearchResultCluster, SearchResultResponse};
use crate::token::AuthToken;

static BASE_URL: &str = "https://mclients.googleapis.com/sj/v2.5/";
static STREAM_URL: &str = "https://mclients.googleapis.com/music/mplay";
pub static CODE_REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

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
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: Option<&str>,
    ) -> Result<GoogleMusicApi, Error> {
        let oauth_client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
            Some(TokenUrl::new(
                "https://www.googleapis.com/oauth2/v3/token".to_string(),
            )?),
        )
        .set_redirect_url(RedirectUrl::new(
            redirect_uri.unwrap_or(CODE_REDIRECT_URI).to_string(),
        )?);

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

    /**
     * Perform an OAuth Login
     *
     * Available handlers:
     * * [auth::stdio_login](auth/fn.stdio_login.html)
     *
     * # Example
     * ```rust,no_run
     * use gmusic::{GoogleMusicApi, auth::stdio_login};
     *
     * let api = GoogleMusicApi::new(String::new(), String::new()).unwrap();
     *
     * api.login(stdio_login).unwrap();
     * ```
     */
    pub fn login<H>(&self, handler: H) -> Result<(), Error>
    where
        H: Fn(String) -> String,
    {
        let token = perform_oauth(&self.client.oauth_client, handler)?;
        self.auth_token.set_token(token);
        Ok(())
    }

    pub fn get_oauth_url(&self) -> (String, String) {
        let (url, verifier) = get_oauth_url(&self.client.oauth_client);

        (url, verifier.secret().clone())
    }

    pub fn request_token(&mut self, code: String, verifier: String) -> Result<(), Error> {
        let verifier = PkceCodeVerifier::new(verifier);

        let token = request_token(&self.client.oauth_client, code, verifier)?;
        self.auth_token.set_token(token);

        Ok(())
    }

    pub fn has_token(&self) -> bool {
        self.auth_token.has_token()
    }

    /**
     * Stores the auth and refresh token in a `.google-auth.json` file for login without user input.
     */
    pub fn store_token(&self) -> Result<(), Error> {
        ensure!(self.auth_token.has_token(), "No token available to persist");
        let token = serde_json::to_string(&self.auth_token.get_token()?)?;
        write(".google-auth.json", token)?; // TODO: configure file path
        Ok(())
    }

    /**
     * Stores the auth and refresh token from a `.google-auth.json` file for login without user input.
     */
    pub fn load_token(&self) -> Result<(), Error> {
        let token = read_to_string(".google-auth.json")?;
        let token = serde_json::from_str(&token)?;
        self.auth_token.set_token(token);
        Ok(())
    }

    /**
     * Returns a list of all user tracks
     */
    // TODO: paging
    pub fn get_all_tracks(&self) -> Result<Vec<Track>, Error> {
        let body = GetAllTracksRequest::new();
        let url = format!("{}trackfeed", BASE_URL);
        let res: GetAllTracksResponse =
            self.api_post(&url, &body, Vec::new(), Vec::new())?.json()?;

        Ok(res.data.items)
    }

    /**
     * Returns a list of all playlists a user has created or subscribed to
     */
    // TODO: paging
    pub fn get_all_playlists(&self) -> Result<Vec<Playlist>, Error> {
        let body = GetAllPlaylistsRequest::new();
        let url = format!("{}playlistfeed", BASE_URL);
        let res: GetAllPlaylistsResponse =
            self.api_post(&url, &body, Vec::new(), Vec::new())?.json()?;

        Ok(res.data.items)
    }

    /**
     * Returns a list of the devices the user has used Google Play Music on
     */
    // TODO: paging
    pub fn get_device_management_info(&self) -> Result<Vec<DeviceManagementInfo>, Error> {
        let url = format!("{}devicemanagementinfo", BASE_URL);
        let res: GetDeviceManagementInfoResponse =
            self.api_get(&url, Vec::new(), Vec::new())?.json()?;

        Ok(res.data.items)
    }

    /**
     * Returns the tracks used in all user created playlists
     */
    pub fn get_playlist_entries(&self) -> Result<Vec<PlaylistEntry>, Error> {
        let mut items = Vec::new();

        let mut res = self.get_playlist_entries_page(None)?;
        items.append(&mut res.data.items);

        let mut next_page_token = res.next_page_token;
        while next_page_token.is_some() {
            let mut res = self.get_playlist_entries_page(next_page_token)?;
            items.append(&mut res.data.items);
            next_page_token = res.next_page_token;
        }

        Ok(items)
    }

    fn get_playlist_entries_page(
        &self,
        page: Option<String>,
    ) -> Result<GetPlaylistEntriesResponse, Error> {
        let url = format!("{}plentryfeed", BASE_URL);
        let request = GetPlaylistEntriesRequest {
            max_results: Some(String::from("20000")),
            start_token: page,
        };
        let mut res: GetPlaylistEntriesResponse = self
            .api_post(&url, &request, Vec::new(), Vec::new())?
            .json()?;

        for entry in &mut res.data.items {
            if let Some(mut track) = entry.track.as_mut() {
                track.id = entry.track_id.clone()
            }
        }

        Ok(res)
    }

    pub fn get_store_track(&self, track_id: &str) -> Result<Track, Error> {
        ensure!(track_id.starts_with("T"), "track_id is not a store id");
        let params = vec![("alt", "json"), ("nid", track_id)];
        let url = format!("{}fetchtrack", BASE_URL);
        let track: Track = self.api_get(&url, Vec::new(), params)?.json()?;

        Ok(track)
    }

    pub fn get_album(&self, album_id: &str) -> Result<Album, Error> {
        let params = vec![
            ("alt", "json"),
            ("nid", album_id),
            ("include-tracks", "true"),
        ];
        let url = format!("{}fetchalbum", BASE_URL);
        let album: Album = self.api_get(&url, Vec::new(), params)?.json()?;

        Ok(album)
    }

    pub fn get_artist(&self, artist_id: &str) -> Result<Artist, Error> {
        let params = vec![("alt", "json"), ("nid", artist_id)];
        let url = format!("{}fetchartist", BASE_URL);
        let artist: Artist = self.api_get(&url, Vec::new(), params)?.json()?;

        Ok(artist)
    }

    /**
     * Get a stream url for the given track id with the given device id
     *
     * Valid for 1 Minute
     */
    pub fn get_stream_url(&self, id: &str, device_id: &str) -> Result<Url, Error> {
        let (sig, salt) = GoogleMusicApi::get_signature(id)?;
        let mut params = vec![
            ("opt", "hi"),
            ("net", "mob"),
            ("pt", "e"),
            ("slt", &salt),
            ("sig", &sig),
        ];
        if id.starts_with("T") {
            params.push(("mjck", id));
        } else {
            params.push(("songid", id));
        }
        let headers = vec![("X-Device-ID", device_id)];
        let res = self.api_get(STREAM_URL, headers, params)?;

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

    pub fn search(
        &self,
        query: &str,
        max_results: Option<u64>,
    ) -> Result<Vec<SearchResultCluster>, Error> {
        let url = format!("{}query", BASE_URL);
        let max_results = max_results.unwrap_or(50);
        let max_results = format!("{}", max_results);
        let params = vec![
            ("ct", "1,2,3,4,5,6,7,8,9"),
            ("ic", "true"),
            ("q", query),
            ("max-results", &max_results),
        ];
        let res: SearchResultResponse = self.api_get(&url, Vec::new(), params)?.json()?;

        Ok(res.cluster_detail)
    }

    fn api_get(
        &self,
        url: &str,
        headers: Vec<(&'static str, &str)>,
        params: Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        self.request::<()>(url, Method::GET, None, headers, params)
    }

    fn api_post<B>(
        &self,
        url: &str,
        body: &B,
        headers: Vec<(&'static str, &str)>,
        params: Vec<(&str, &str)>,
    ) -> Result<Response, Error>
    where
        B: serde::Serialize,
    {
        self.request(url, Method::POST, Some(body), headers, params)
    }

    fn request<B>(
        &self,
        url: &str,
        method: Method,
        body: Option<&B>,
        headers: Vec<(&'static str, &str)>,
        params: Vec<(&str, &str)>,
    ) -> Result<Response, Error>
    where
        B: serde::Serialize,
    {
        if self.auth_token.requires_new_token() {
            self.auth_token.refresh(&self.client.oauth_client)?;
        }
        let client = reqwest::Client::new();
        let mut url = Url::parse(url)?;
        {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in GoogleMusicApi::default_params() {
                query_pairs.append_pair(key, value);
            }
            for (key, value) in params {
                query_pairs.append_pair(key, value);
            }
        }
        let mut req = client.request(method, url);
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            header_map.insert(key, value.parse()?);
        }
        req = req.headers(header_map);
        if let Some(body) = body {
            req = req.json(&body);
        }
        let res = req
            .try_clone()
            .unwrap()
            .header(AUTHORIZATION, self.auth_token.get_auth_header()?)
            .send()?
            .error_for_status()
            .or_else(|err| self.retry_request(req, err))?;

        Ok(res)
    }

    fn retry_request(&self, req: RequestBuilder, err: reqwest::Error) -> Result<Response, Error> {
        if let Some(StatusCode::UNAUTHORIZED) = err.status() {
            self.auth_token.refresh(&self.client.oauth_client)?;
            let res = req
                .header(AUTHORIZATION, self.auth_token.get_auth_header()?)
                .send()?
                .error_for_status()?;
            Ok(res)
        } else {
            Err(err.into())
        }
    }

    fn default_params() -> Vec<(&'static str, &'static str)> {
        vec![("dv", "0"), ("hl", "en_US"), ("tier", "aa")]
    }
}
