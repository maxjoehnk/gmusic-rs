use crate::models::album::Album;
use crate::models::artist::Artist;
use crate::{Playlist, Track};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultResponse {
    pub kind: String,
    pub cluster_detail: Vec<SearchResultCluster>,
    pub suggested_query: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultCluster {
    pub cluster: SearchResultClusterInfo,
    pub display_name: Option<String>,
    #[serde(default)]
    pub entries: Vec<SearchResult>,
    pub result_token: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultClusterInfo {
    pub category: String,
    pub id: String,
    #[serde(rename = "type")]
    pub cluster_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub score: Option<f64>,
    #[serde(rename = "type")]
    pub result_type: String,
    pub best_result: Option<bool>,
    pub navigational_result: Option<bool>,
    pub navigational_confidence: Option<f64>,
    pub cluster: Vec<SearchResultClusterInfo>,
    pub track: Option<Track>,
    pub playlist: Option<Playlist>,
    pub artist: Option<Artist>,
    pub album: Option<Album>,
}
