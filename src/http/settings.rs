#[derive(Debug, Serialize)]
pub struct GetSettingsRequest {
    #[serde(rename = "sessionId")]
    session_id: String
}

impl GetSettingsRequest {
    pub fn new() -> GetSettingsRequest {
        GetSettingsRequest {
            session_id: String::new()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetSettingsResponse {
    pub response: GetSettingsPayload
}

#[derive(Debug, Deserialize)]
pub struct GetSettingsPayload {
    pub settings: Settings
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(rename = "entitlementInfo")]
    pub entitlement_info: SettingsEntitlementInfo,
    #[serde(rename = "uploadDevice")]
    pub upload_device: Vec<UploadDevice>
}

#[derive(Debug, Deserialize)]
pub struct SettingsEntitlementInfo {
    #[serde(rename = "isSubscription")]
    pub is_subscription: bool
}

#[derive(Debug, Deserialize)]
pub struct UploadDevice {
    #[serde(rename = "deviceType")]
    pub device_type: i32,
    pub id: String
}