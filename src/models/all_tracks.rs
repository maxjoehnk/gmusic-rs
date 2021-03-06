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
    #[serde(default)]
    pub artist_id: Vec<String>,
    pub album_id: Option<String>,
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
    pub comment: Option<String>,
    pub year: Option<u64>,
    pub total_disc_count: Option<u64>,
    pub disc_number: Option<u64>,
    pub beats_per_minute: Option<u64>,
    pub genre: Option<String>,
    #[serde(default)]
    pub play_count: u64,
    pub rating: Option<TrackRating>
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum TrackRating {
    #[serde(rename = "0")]
    None,
    #[serde(rename = "5")]
    Like,
    #[serde(rename = "1")]
    Dislike
}

impl GetAllTracksRequest {
    pub fn new() -> GetAllTracksRequest {
        GetAllTracksRequest
    }
}
