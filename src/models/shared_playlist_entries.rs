use serde::{Deserialize, Serialize};

use crate::Track;
use crate::models::playlist_entries::GetPlaylistEntriesRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPlaylistEntriesResponse {
    pub kind: String,
    pub entries: Vec<SharedPlaylist>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPlaylist {
    pub share_token: String,
    pub response_code: String,
    #[serde(default)]
    pub playlist_entry: Vec<SharedPlaylistEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPlaylistEntry {
    pub kind: String,
    pub id: String,
    pub absolute_position: String,
    pub track_id: String,
    pub creation_timestamp: String,
    pub last_modified_timestamp: String,
    pub deleted: bool,
    pub source: String,
    #[serde(default)]
    pub track: Option<Track>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SharedPlaylistContentsRequest {
    pub entries: Vec<SharedPlaylistContentsFilter>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPlaylistContentsFilter {
    pub share_token: String,
    #[serde(flatten, default)]
    pub pages: GetPlaylistEntriesRequest,
}
