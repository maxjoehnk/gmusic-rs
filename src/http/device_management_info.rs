use serde_derive::Deserialize;

use crate::http::GMusicListResponse;

pub type GetDeviceManagementInfoResponse = GMusicListResponse<DeviceManagementInfo>;

#[derive(Debug, Deserialize, Clone)]
pub struct DeviceManagementInfo {
    pub id: String,
    #[serde(rename = "friendlyName", default)]
    pub friendly_name: Option<String>,
    #[serde(rename = "lastAccessedTimeMs")]
    pub last_accessed_time_ms: String,
    #[serde(rename = "smartPhone", default)]
    pub smart_phone: Option<bool>,
    #[serde(rename = "type")]
    pub device_type: String,
    pub kind: String,
}
