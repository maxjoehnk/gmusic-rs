use serde::{Deserialize, Serialize};

use crate::models::image_ref::ImageRef;
use crate::models::GMusicListResponse;

#[derive(Debug, Serialize)]
pub struct GetAllTracksRequest;

pub type GetAllTracksResponse = GMusicListResponse<Track>;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    #[serde(default)]
    pub album_artist: Option<String>,
    pub track_number: u64,
    #[serde(default)]
    pub total_track_count: Option<u64>,
    pub duration_millis: String,
    #[serde(default)]
    pub album_art_ref: Vec<ImageRef>,
    #[serde(default)]
    pub artist_art_ref: Vec<ImageRef>,
    pub disk_number: Option<u64>,
    pub store_id: Option<String>,
}

impl GetAllTracksRequest {
    pub fn new() -> GetAllTracksRequest {
        GetAllTracksRequest
    }
}
