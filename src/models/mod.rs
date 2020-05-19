use serde::{Deserialize, Serialize};

pub mod album;
pub mod all_playlists;
pub mod all_tracks;
pub mod artist;
pub mod device_management_info;
pub mod image_ref;
pub mod playlist_entries;
pub mod search_results;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GMusicResponse<T> {
    pub data: T,
    pub kind: String,
    pub next_page_token: Option<String>,
}

pub type GMusicListResponse<T> = GMusicResponse<GMusicList<T>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GMusicList<T> {
    pub items: Vec<T>,
}
