use serde::{Deserialize, Serialize};

use crate::models::GMusicListResponse;
use crate::Track;

pub type GetPlaylistEntriesResponse = GMusicListResponse<PlaylistEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEntry {
    pub kind: String,
    pub id: String,
    pub client_id: String,
    pub playlist_id: String,
    pub absolute_position: String,
    pub track_id: String,
    pub creation_timestamp: String,
    pub last_modified_timestamp: String,
    pub deleted: bool,
    pub source: String,
    #[serde(default)]
    pub track: Option<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct GetPlaylistEntriesRequest {
    pub start_token: Option<String>,
    pub max_results: Option<String>,
}
