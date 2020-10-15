//use reqwest::Client;
use futures::executor;
use std::time;
use crate::types::{GetAccessTokenResponse, GetAccessTokenRequest, GetAccessTokenError, HaEntityState, RegisterDeviceResponse, RegisterDeviceRequest, SensorRegistrationRequest, RegisterSensorResponse};

pub mod errors;
pub mod types;

const CLIENT_ID: &str = "https://halcyon.casa";

#[derive(Debug)]
pub struct HomeAssistantAPI {
    instance_urls: Vec<String>,
    token: Option<Token>,
    client: reqwest::Client,
    webhook_id: Option<String>,
    cloudhook_url: Option<String>,
    remote_ui_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Oauth(OAuthToken),
    LongLived(LongLivedToken),
}

#[derive(Debug, Clone)]
pub struct OAuthToken {
    token: String,
    token_expiration: std::time::SystemTime,
    refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct LongLivedToken {
    token: String,
}

impl HomeAssistantAPI {
    pub fn new(instance_urls: Vec<String>) -> Self {
        return Self {
            instance_urls,
            client: reqwest::Client::new(),
            token: None,
            webhook_id: None,
            cloudhook_url: None,
            remote_ui_url: None,
        }
    }

    pub fn single_instance(instance_url: String) -> Self {
        return Self {
            instance_urls: vec![instance_url],
            token: None,
            client: reqwest::Client::new(),
            webhook_id: None,
            cloudhook_url: None,
            remote_ui_url: None
        }
    }

    pub fn set_access_token(self, token: String) -> Self {
        return Self {
            token: Some(Token::LongLived(LongLivedToken{ token})),
            ..self
        }
    }

    pub fn set_webhook_id(self, webhook_id: String) -> Self {
        return Self {
            webhook_id: Some(webhook_id),
            ..self
        }
    }

    pub async fn access_token(&self, code: String) -> Result<GetAccessTokenResponse, errors::Error> {
        let request = GetAccessTokenRequest {
            grant_type: "authorization_code".to_string(),
            code,
            client_id: "http://localhost:8000".to_string(),
        };
        let resp = self.client
            .post(format!("http://{}/auth/token", self.instance_urls[0]).as_str())
            .form(&request)
            .send()
            .await?;

        match resp.status().as_str() {
            "200" => Ok(resp.json::<GetAccessTokenResponse>().await?),
            _ => {
                let error = resp.json::<GetAccessTokenError>().await?;
                Err(errors::Error::HaApi(format!(
                    "Error getting access token from HA Error: {} Details: {}",
                    error.error, error.error_description
                )))
            }
        }
    }

    pub async fn api_states(&self) -> Result<Vec<HaEntityState>, errors::Error> {
        let endpoint = format!("http://{}/api/states", self.instance_urls[0]);
        let token = self.get_token()?;
        let resp = self.client
            .get(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let api_states = resp.json::<Vec<HaEntityState>>().await?;
        Ok(api_states)
    }

    pub async fn register_machine(
        &self,
        request: &RegisterDeviceRequest
    ) -> Result<RegisterDeviceResponse, errors::Error> {

        let endpoint = format!("http://{}/api/mobile_app/registrations", self.instance_urls[0]);
        let token = self.get_token()?;
        let resp = self
            .client
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", token),
            )
            .json(&request)
            .send()
            .await?;

        let r: RegisterDeviceResponse = resp.json().await?;
        Ok(r)
    }

    pub async fn register_sensor(
        &self,
        request: &SensorRegistrationRequest,
    ) -> Result<RegisterSensorResponse, errors::Error> {

        let webhook_id = self.webhook_id.as_ref().ok_or_else(|| errors::Error::Config("expected webhook_id to exist".to_string()))?;
        let token = self.get_token()?;
        let endpoint = format!("http://{}/api/webhook/{}", self.instance_urls[0], webhook_id);

        let response = self
            .client
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", token),
            )
            .json(&request)
            .send()
            .await?;

        let resp_json: RegisterSensorResponse =  response.json().await?;
        Ok(resp_json)
    }

    fn get_token(&self) -> Result<String, errors::Error> {
        let token = self.token.as_ref().ok_or_else(|| errors::Error::Config("expected a token to exist".to_string()))?;
        match token {
            Token::Oauth(token) => Ok(token.token.clone()),
            Token::LongLived(token) => Ok(token.token.clone())
        }
    }

}
