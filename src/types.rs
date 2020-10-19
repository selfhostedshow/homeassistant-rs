use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HaEntityAttribute {
    pub friendly_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HaEntityState {
    pub attributes: HaEntityAttribute,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterDeviceRequest {
    pub device_id: String,
    pub app_id: String,
    pub app_name: String,
    pub app_version: String,
    pub device_name: String,
    pub manufacturer: String,
    pub model: String,
    pub os_name: String,
    pub os_version: String,
    pub supports_encryption: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterDeviceResponse {
    pub cloud_hook_url: Option<String>,
    pub remote_ui_url: Option<String>,
    pub secret: Option<String>,
    pub webhook_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccessTokenRequest {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccessTokenError {
    pub error: String,
    pub error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorRegistrationRequest {
    pub r#type: String,
    pub data: SensorRegistrationData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorRegistrationData {
    pub device_class: Option<String>,
    pub icon: String,
    pub name: String,
    pub state: String,
    pub r#type: String,
    pub unique_id: String,
    pub unit_of_measurement: String,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterSensorResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorUpdateRequest {
    pub r#type: String,
    pub data: SensorUpdateData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorUpdateData {
    pub icon: String,
    pub state: String,
    pub r#type: String,
    pub unique_id: String,
    pub attributes: std::collections::HashMap<String, String>,
}
