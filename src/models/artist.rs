use crate::Track;
use serde_derive::Deserialize;
use crate::models::album::Album;
use crate::models::image_ref::ImageRef;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "artistId")]
    pub id: String,
    pub kind: String,
    pub name: String,
    pub artist_art_ref: Option<String>,
    pub artist_art_refs: Vec<ImageRef>,
    pub artist_bio: Option<String>,
    #[serde(default)]
    pub albums: Vec<Album>,
    #[serde(default)]
    pub top_tracks: Vec<Track>,
    #[serde(rename = "total_albums")]
    pub total_albums: Option<u64>,
}