use crate::models::GMusicResponse;
use crate::Track;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "albumId")]
    id: String,
    kind: String,
    name: String,
    album_artist: String,
    album_art_ref: Option<String>,
    artist: String,
    artist_id: Vec<String>,
    year: Option<u64>,
    #[serde(default)]
    tracks: Vec<Track>,
    description: Option<String>,
    explicit_type: String,
    content_type: Option<String>
}