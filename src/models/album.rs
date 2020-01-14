use crate::models::GMusicResponse;
use crate::Track;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "albumId")]
    pub id: String,
    pub kind: String,
    pub name: String,
    pub album_artist: String,
    pub album_art_ref: Option<String>,
    pub artist: String,
    pub artist_id: Vec<String>,
    pub year: Option<u64>,
    #[serde(default)]
    pub tracks: Vec<Track>,
    pub description: Option<String>,
    pub explicit_type: String,
    pub content_type: Option<String>
}