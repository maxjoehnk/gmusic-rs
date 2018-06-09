#[derive(Debug, Serialize)]
pub struct GetAllTracksRequest {
    limit: u32
}

#[derive(Debug, Deserialize)]
pub struct GetAllTracksResponse {
    pub kind: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
    pub data: GetAllTracksData
}

#[derive(Debug, Deserialize)]
pub struct GetAllTracksData {
    pub items: Vec<Track>
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String
}

impl GetAllTracksRequest {
    pub fn new(limit: u32) -> GetAllTracksRequest {
        GetAllTracksRequest {
            limit
        }
    }
}