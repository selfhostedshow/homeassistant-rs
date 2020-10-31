use crate::types;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};
use crate::errors;


#[derive(Serialize, Deserialize)]
pub struct NativeAppConfig {
    webhook_id: Option<String>,
    cloudhook_url: Option<String>,
    remote_ui_url: Option<String>,
    secret: Option<String>
}



#[derive(Debug)]
pub struct NativeApp { 
    webhook_id: Option<String>,
    cloudhook_url: Option<String>,
    remote_ui_url: Option<String>,
    secret: Option<String>,
    ha_client: Arc<RwLock<crate::HomeAssistantAPI>>,
}

impl NativeApp {

    pub fn new(ha_client: Weak<RwLock<crate::HomeAssistantAPI>>) -> Result<Self, errors::Error>{
        match ha_client.upgrade() {
            Some(ha_api) => 
                Ok(Self {
                    webhook_id: None,
                    cloudhook_url: None,
                    remote_ui_url: None,
                    secret: None,
                    ha_client: ha_api,
                }),
            None => Err(errors::Error::HaApi(String::from("Weak PTR Upgrade unsececcful")))
        }
    }

    pub fn from_config(config: NativeAppConfig, ha_client: Weak<RwLock<crate::HomeAssistantAPI>>) -> Result<Self, errors::Error>  {
        match ha_client.upgrade() {
            Some(ha_api) => 
                Ok(Self {
                    webhook_id: config.webhook_id,
                    cloudhook_url: config.cloudhook_url,
                    remote_ui_url: config.remote_ui_url,
                    secret: config.secret,
                    ha_client: ha_api,
                })
            ,
            None => Err(errors::Error::HaApi(String::from("Weak PTR Upgrade unsececcful")))
        }
    }

    pub fn set_webhook_info(
        &mut self,
        webhook_id: String,
        cloudhook_url: Option<String>,
        remote_ui_url: Option<String>,
    ) {
        self.webhook_id = Some(webhook_id);
        self.cloudhook_url = cloudhook_url;
        self.remote_ui_url = remote_ui_url;
    }

    pub async fn register_machine(
        &mut self,
        request: &types::RegisterDeviceRequest,
    ) -> Result<types::RegisterDeviceResponse, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();//panic
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }
        let endpoint = format!("http://{}/api/mobile_app/registrations", read_lock.instance_url);
        let resp = reqwest::Client::new()
            .post(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", read_lock.token.as_string()?))
            .json(&request)
            .send()
            .await?;
        drop(read_lock);
        let r: types::RegisterDeviceResponse = resp.json().await?;
        self.set_webhook_info(
            r.webhook_id.clone(),
            r.cloud_hook_url.clone(),
            r.remote_ui_url.clone(),
        );
        Ok(r)
    }

    pub async fn register_sensor(
        &mut self,
        request: &types::SensorRegistrationRequest,
    ) -> Result<types::RegisterSensorResponse, errors::Error> {

        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }
        let webhook_id = self
            .webhook_id
            .as_ref()
            .ok_or_else(|| errors::Error::Config("expected webhook_id to exist".to_string()))?;
        let endpoint = format!("http://{}/api/webhook/{}", read_lock.instance_url, webhook_id);

        let response = reqwest::Client::new()
            .post(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", read_lock.token.as_string()?))
            .json(&request)
            .send()
            .await?;
        drop(read_lock);
        let resp_json: types::RegisterSensorResponse = response.json().await?;
        Ok(resp_json)
    }

    pub async fn update_sensor(
        &mut self,
        sensor_data: types::SensorUpdateData,
    ) -> Result<(), errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();//panic
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }
        let webhook_id = self
            .webhook_id
            .as_ref()
            .ok_or_else(|| errors::Error::Config("missing webhook id".to_string()))?;

        let endpoint = format!("{}/api/webhook/{}", read_lock.instance_url, webhook_id);

        let request = crate::types::SensorUpdateRequest {
            data: sensor_data,
            r#type: String::from("update_sensor_states"),
        };

        reqwest::Client::new()
            .post(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", read_lock.token.as_string()?))
            .json(&request)
            .send()
            .await?;
            
        drop(read_lock);

        Ok(())
    }
}