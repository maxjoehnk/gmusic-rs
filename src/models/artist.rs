use crate::models::GMusicResponse;
use crate::Track;
use serde_derive::Deserialize;
use crate::models::album::Album;
use crate::models::image_ref::ImageRef;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "artistId")]
    id: String,
    kind: String,
    name: String,
    artist_art_ref: Option<String>,
    artist_art_refs: Vec<ImageRef>,
    artist_bio: String,
    #[serde(default)]
    albums: Vec<Album>,
    #[serde(default)]
    top_tracks: Vec<Track>,
    #[serde(rename = "total_albums")]
    total_albums: Option<u64>,
}