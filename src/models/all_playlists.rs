use serde_derive::{Deserialize, Serialize};

use crate::models::image_ref::ImageRef;
use crate::models::GMusicListResponse;

#[derive(Debug, Serialize)]
pub struct GetAllPlaylistsRequest;

pub type GetAllPlaylistsResponse = GMusicListResponse<Playlist>;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub deleted: Option<bool>,
    #[serde(rename = "type")]
    pub playlist_type: PlaylistType,
    pub last_modified_timestamp: Option<String>,
    pub recent_timestamp: Option<String>,
    pub share_token: String,
    pub owner_profile_photo_url: Option<String>,
    pub owner_name: Option<String>,
    pub access_controlled: Option<bool>,
    pub share_state: Option<PlaylistShareState>,
    pub creation_timestamp: Option<String>,
    #[serde(default)]
    pub album_art_ref: Vec<ImageRef>,
    pub description: Option<String>,
    pub explicit_type: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaylistType {
    Magic,
    Shared,
    UserGenerated,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaylistShareState {
    Private,
    Public,
}

impl GetAllPlaylistsRequest {
    pub fn new() -> GetAllPlaylistsRequest {
        GetAllPlaylistsRequest
    }
}
