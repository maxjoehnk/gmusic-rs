use serde_derive::{Serialize, Deserialize};

pub mod device_management_info;
pub mod all_tracks;
pub mod all_playlists;
pub mod playlist_entries;
pub mod image_ref;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GMusicResponse<T> {
    pub data: T,
    pub kind: String
}

pub type GMusicListResponse<T> = GMusicResponse<GMusicList<T>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GMusicList<T> {
    pub items: Vec<T>
}

