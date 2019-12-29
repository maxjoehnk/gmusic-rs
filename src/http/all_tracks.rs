use serde_derive::{Deserialize, Serialize};

use crate::http::GMusicListResponse;

#[derive(Debug, Serialize)]
pub struct GetAllTracksRequest;

pub type GetAllTracksResponse = GMusicListResponse<Track>;

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
}

impl GetAllTracksRequest {
    pub fn new() -> GetAllTracksRequest {
        GetAllTracksRequest
    }
}