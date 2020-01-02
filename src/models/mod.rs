use serde_derive::{Deserialize, Serialize};

pub mod all_playlists;
pub mod all_tracks;
pub mod device_management_info;
pub mod image_ref;
pub mod playlist_entries;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GMusicResponse<T> {
    pub data: T,
    pub kind: String,
}

pub type GMusicListResponse<T> = GMusicResponse<GMusicList<T>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GMusicList<T> {
    pub items: Vec<T>,
}
