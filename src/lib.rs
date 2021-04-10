use crate::types::*;
use std::convert::TryFrom;
use std::sync::{Arc, RwLock, Weak};
use std::time;

pub mod errors;
pub mod native_app;
pub mod rest;
pub mod types;

#[derive(Debug)]
pub struct HomeAssistantAPI {
    instance_url: String,
    token: Token,
    client_id: String,
    self_reference: Weak<RwLock<Self>>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Oauth(OAuthToken),
    LongLived(LongLivedToken),
    None,
}

impl Token {
    pub fn as_string(&self) -> Result<String, errors::Error> {
        match self {
            Token::Oauth(token) => Ok(token.token.clone()),
            Token::LongLived(token) => Ok(token.token.clone()),
            Token::None => Err(errors::Error::NoAuth()),
        }
    }

    pub fn need_refresh(&self) -> bool {
        match self {
            Token::Oauth(token) => {
                match time::SystemTime::now().duration_since(token.token_expiration) {
                    Ok(sec_left) => sec_left > time::Duration::from_secs(10),
                    Err(_) => false,
                }
            }
            Token::LongLived(_) => false,
            Token::None => false,
        }
    }
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
    pub fn new(instance_url: String, client_id: String) -> Arc<RwLock<Self>> {
        let token = Token::None;
        let http_instance_url = format!("http://{}", instance_url);
        let ret = Arc::new(RwLock::new(Self {
            instance_url: http_instance_url,
            token,
            client_id,
            self_reference: Weak::new(),
        }));

        ret.write().unwrap().self_reference = Arc::downgrade(&ret);

        ret
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
        self.token = Token::Oauth(oauth);
    }

    pub fn set_long_lived_token(&mut self, token: String) {
        self.token = Token::LongLived(LongLivedToken { token });
    }

    pub async fn refresh_oauth_token(&mut self) -> Result<(), errors::Error> {
        let refresh_token = String::from("test");

        let response = reqwest::Client::new()
            .post(format!("{}/auth/token", self.instance_url).as_str())
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
        let resp = reqwest::Client::new()
            .post(format!("{}/auth/token", self.instance_url).as_str())
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

    pub async fn get_rest_client(&self) -> rest::Rest {
        match rest::Rest::try_from(self.self_reference.clone()) {
            Ok(rest) => rest,
            Err(_) => unreachable!(),
        }
    }

    pub async fn get_native_client_from_config(
        &self,
        config: native_app::NativeAppConfig,
    ) -> native_app::NativeApp {
        match native_app::NativeApp::from_config(config, self.self_reference.clone()) {
            Ok(native_app) => native_app,
            Err(_) => unreachable!(),
        }
    }

    pub async fn get_native_client(&self) -> native_app::NativeApp {
        match native_app::NativeApp::new(self.self_reference.clone()) {
            Ok(native_app) => native_app,
            Err(_) => unreachable!(),
        }
    }
}
