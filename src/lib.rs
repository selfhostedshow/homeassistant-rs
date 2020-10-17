use crate::types::*;
use std::time;

pub mod errors;
pub mod types;

#[derive(Debug)]
pub struct HomeAssistantAPI {
    instance_url: String,
    token: Option<Token>,
    client: reqwest::Client,
    webhook_id: Option<String>,
    cloudhook_url: Option<String>,
    remote_ui_url: Option<String>,
    client_id: String,
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
    pub fn new(instance_url: String, client_id: String) -> Self {
        Self {
            instance_url,
            token: None,
            client: reqwest::Client::new(),
            webhook_id: None,
            cloudhook_url: None,
            remote_ui_url: None,
            client_id,
        }
    }

    pub fn set_oauth_token(
        &mut self,
        access_token: String,
        expires_in: u32,
        refresh_token: String,
    ) {
        let oauth = OAuthToken {
            token: access_token,
            token_expiration: time::SystemTime::now()
                + time::Duration::from_secs(expires_in as u64),
            refresh_token,
        };
        let oauth_token = Token::Oauth(oauth);
        self.token = Some(oauth_token);
    }

    pub fn set_long_lived_token(&mut self, token: String) {
        let long_lived = Token::LongLived(LongLivedToken { token });
        self.token = Some(long_lived)
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

    pub fn need_refresh(&self) -> bool {
        let token_result = self
            .token
            .as_ref()
            .ok_or_else(|| errors::Error::Config("expected a token to exist".to_string()));

        match token_result {
            Ok(token) => match token {
                Token::Oauth(token) => {
                    match time::SystemTime::now().duration_since(token.token_expiration) {
                        Ok(sec_left) => sec_left > time::Duration::from_secs(10),
                        Err(_) => false,
                    }
                }
                Token::LongLived(_) => false,
            },
            Err(_) => false,
        }
    }

    pub async fn refresh_token(&mut self) -> Result<(), errors::Error> {
        let token = self.token.clone(); // This is dump but I have to do it apparently
        let refresh_token: String;
        match token {
            Some(token) => {
                refresh_token = match token {
                    Token::Oauth(oauth) => oauth.refresh_token,
                    Token::LongLived(_) => {
                        return Err(errors::Error::Refresh());
                    }
                };
            }
            None => return Err(errors::Error::NoAuth()),
        }

        let response = self
            .client
            .post(format!("http://{}/auth/token", self.instance_url).as_str())
            .query(&[
                ("grant_type", "refresh_token"),
                ("client_id", &self.client_id),
                ("refresh_token", refresh_token.as_str()),
            ])
            .send()
            .await?;

        let refresh_token_resp: RefreshAccessTokenResponse = response.json().await?;
        self.set_oauth_token(
            refresh_token_resp.access_token,
            refresh_token_resp.expires_in,
            refresh_token,
        );
        Ok(())
    }

    pub async fn access_token(
        &mut self,
        code: String,
        client_id: String,
    ) -> Result<GetAccessTokenResponse, errors::Error> {
        let request = GetAccessTokenRequest {
            grant_type: "authorization_code".to_string(),
            code,
            client_id,
        };
        let resp = self
            .client
            .post(format!("http://{}/auth/token", self.instance_url).as_str())
            .form(&request)
            .send()
            .await?;

        match resp.status().as_str() {
            "200" => {
                let access_token_resp = resp.json::<GetAccessTokenResponse>().await?;
                self.set_oauth_token(
                    access_token_resp.access_token.clone(),
                    access_token_resp.expires_in,
                    access_token_resp.refresh_token.clone(),
                );
                Ok(access_token_resp)
            }
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
        let endpoint = format!("http://{}/api/states", self.instance_url);
        let token = self.get_token()?;
        let resp = self
            .client
            .get(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let api_states = resp.json::<Vec<HaEntityState>>().await?;
        Ok(api_states)
    }

    pub async fn register_machine(
        &mut self,
        request: &RegisterDeviceRequest,
    ) -> Result<RegisterDeviceResponse, errors::Error> {
        if self.need_refresh() {
            self.refresh_token().await?;
        }
        let endpoint = format!("http://{}/api/mobile_app/registrations", self.instance_url);
        let token = self.get_token()?;
        let resp = self
            .client
            .post(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;

        let r: RegisterDeviceResponse = resp.json().await?;
        self.set_webhook_info(
            r.webhook_id.clone(),
            r.cloud_hook_url.clone(),
            r.remote_ui_url.clone(),
        );
        Ok(r)
    }

    pub async fn register_sensor(
        &mut self,
        request: &SensorRegistrationRequest,
    ) -> Result<RegisterSensorResponse, errors::Error> {
        if self.need_refresh() {
            self.refresh_token().await?;
        }
        let webhook_id = self
            .webhook_id
            .as_ref()
            .ok_or_else(|| errors::Error::Config("expected webhook_id to exist".to_string()))?;
        let token = self.get_token()?;
        let endpoint = format!("http://{}/api/webhook/{}", self.instance_url, webhook_id);

        let response = self
            .client
            .post(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;

        let resp_json: RegisterSensorResponse = response.json().await?;
        Ok(resp_json)
    }

    pub async fn update_sensor(
        &mut self,
        sensor_data: SensorUpdateData,
    ) -> Result<(), errors::Error> {
        if self.need_refresh() {
            self.refresh_token().await?;
        }
        let webhook_id = self
            .webhook_id
            .as_ref()
            .ok_or_else(|| errors::Error::Config("missing webhook id".to_string()))?;

        let endpoint = format!("{}/api/webhook/{}", self.instance_url, webhook_id);
        let token = self.get_token()?;

        let request = types::SensorUpdateRequest {
            data: sensor_data,
            r#type: String::from("update_sensor_states"),
        };

        self.client
            .post(endpoint.as_str())
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await?;

        Ok(())
    }

    fn get_token(&self) -> Result<String, errors::Error> {
        let token = self
            .token
            .as_ref()
            .ok_or_else(|| errors::Error::Config("expected a token to exist".to_string()))?;
        match token {
            Token::Oauth(token) => Ok(token.token.clone()),
            Token::LongLived(token) => Ok(token.token.clone()),
        }
    }
}
