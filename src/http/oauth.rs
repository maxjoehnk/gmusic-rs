use auth::encrypt_login;

#[derive(Debug, Serialize)]
pub struct OAuthRequest {
    #[serde(rename = "accountType")]
    account_type: String,
    has_permission: u32,
    service: String,
    source: String,
    #[serde(rename = "androidId")]
    android_id: String,
    app: String,
    device_country: String,
    #[serde(rename = "operatorCountry")]
    operator_country: String,
    lang: String,
    sdk_version: String,
    #[serde(rename = "Token")]
    master_token: Option<String>,
    #[serde(rename = "EncryptedPasswd")]
    encrypted_password: Option<String>,
    #[serde(rename = "Email")]
    email: Option<String>
}

impl OAuthRequest {
    pub fn from_token(android_id: String, master_token: String) -> OAuthRequest {
        OAuthRequest {
            account_type: "HOSTED_OR_GOOGLE".to_owned(),
            has_permission: 1,
            service: "sj".to_owned(),
            source: "android".to_owned(),
            android_id,
            app: "com.google.android.music".to_owned(),
            device_country: "us".to_owned(),
            operator_country: "us".to_owned(),
            lang: "en".to_owned(),
            sdk_version: "17".to_owned(),
            master_token: Some(master_token),
            encrypted_password: None,
            email: None
        }
    }

    pub fn from_userdata(android_id: String, email: String, password: String) -> OAuthRequest {
        OAuthRequest {
            account_type: "HOSTED_OR_GOOGLE".to_owned(),
            has_permission: 1,
            service: "sj".to_owned(),
            source: "android".to_owned(),
            android_id,
            app: "com.google.android.music".to_owned(),
            device_country: "us".to_owned(),
            operator_country: "us".to_owned(),
            lang: "en".to_owned(),
            sdk_version: "17".to_owned(),
            master_token: None,
            encrypted_password: Some(encrypt_login(email.clone(), password)),
            email: Some(email)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthResponse {
    #[serde(rename = "Auth")]
    pub auth: String
}