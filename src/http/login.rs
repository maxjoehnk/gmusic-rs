use auth::encrypt_login;

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    #[serde(rename = "accountType")]
    account_type: String,
    #[serde(rename = "Email")]
    email: String,
    has_permission: String,
    add_account: String,
    #[serde(rename = "EncryptedPasswd")]
    encrypted_password: String,
    service: String,
    source: String,
    #[serde(rename = "androidId")]
    pub android_id: String,
    device_country: String,
    #[serde(rename = "operatorCountry")]
    operator_country: String,
    lang: String,
    sdk_version: String
}

impl LoginRequest {
    pub fn new(email: String, password: String, android_id: String) -> LoginRequest {
        let encrypted_password = encrypt_login(email.clone(), password);
        LoginRequest {
            account_type: String::from("HOSTED_OR_GOOGLE"),
            email,
            has_permission: String::from("1"),
            add_account: String::from("1"),
            encrypted_password,
            service: String::from("ac2dm"),
            source: String::from("android"),
            android_id,
            device_country: String::from("us"),
            operator_country: String::from("us"),
            lang: String::from("en"),
            sdk_version: String::from("17")
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "Auth")]
    pub auth: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "GooglePlusUpdate")]
    pub google_plus_update: u32,
    #[serde(rename = "LSID")]
    pub lsid: String,
    #[serde(rename = "SID")]
    pub sid: String,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub services: String
}
