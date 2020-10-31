use crate::errors;
use crate::types;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::{Arc, RwLock, Weak};

#[derive(Debug)]
pub struct Rest {
    ha_client: Arc<RwLock<crate::HomeAssistantAPI>>,
}

impl Rest {
    pub async fn check(self) -> Result<String, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/config", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        #[derive(Serialize, Deserialize, Debug)]
        struct Response {
            message: String,
        }

        let resp_json: Response = response.json().await?;

        return Ok(resp_json.message);

        //Err(errors::Error::HaApi(String::from("Not Implemented")))
    }

    pub async fn config(self) -> Result<types::Configuration, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/config", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        let resp_json: types::Configuration = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn discovery_info(self) -> Result<types::DiscoveryInfo, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/discovery_info", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        let resp_json: types::DiscoveryInfo = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn events(self) -> Result<Vec<types::EventObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/events", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        let resp_json: Vec<types::EventObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn services(self) -> Result<Vec<types::ServiceObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/services", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        let resp_json: Vec<types::ServiceObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn history_period(
        self,
        timestamp: Option<chrono::DateTime<chrono::Utc>>,
        filter_entity_id: Option<String>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        significant_changes_only: Option<bool>,
    ) -> Result<Vec<types::StateObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let mut endpoint = format!("{}/api/history/period", read_lock.instance_url);

        match timestamp {
            Some(timestamp) => {
                let formatted_timestamp = timestamp.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                endpoint = endpoint + &formatted_timestamp;
            }
            None => {}
        };

        let mut request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        match filter_entity_id {
            Some(filter_entity_id) => {
                request = request.query(&[("filter_entity_id", filter_entity_id)]);
            }
            None => {}
        };

        match end_time {
            Some(end_time) => {
                let formatted_timestamp = end_time.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                request = request.query(&[(
                    "end_time",
                    formatted_timestamp,
                )]);
            }
            None => {}
        };

        match significant_changes_only {
            Some(_) => {
                request = request.query(&[("significant_changes_only")]);
            }
            None => {}
        };

        let response = request.send().await?;

        let resp_json: Vec<types::StateObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn history_period_minimal(
        self,
        timestamp: Option<chrono::DateTime<chrono::Utc>>,
        filter_entity_id: Option<String>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        significant_changes_only: Option<bool>,
    ) -> Result<Vec<types::StateObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let mut endpoint = format!("{}/api/history/period", read_lock.instance_url);

        match timestamp {
            Some(timestamp) => {
                let formatted_timestamp = timestamp.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                endpoint = endpoint + &formatted_timestamp.to_string();
            }
            None => {}
        };

        let mut request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        match filter_entity_id {
            Some(filter_entity_id) => {
                request = request.query(&[("filter_entity_id", filter_entity_id)]);
            }
            None => {}
        };

        match end_time {
            Some(end_time) => {
                let formatted_timestamp = end_time.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                request = request.query(&[("end_time", formatted_timestamp)]);
            }
            None => {}
        };

        match significant_changes_only {
            Some(_) => {
                request = request.query(&[("significant_changes_only")]);
            }
            None => {}
        };

        request = request.query(&[("minimal_response")]);

        let response = request.send().await?;

        let resp_json: Vec<types::StateObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn logbook(
        self,
        timestamp: Option<chrono::DateTime<chrono::Utc>>,
        entity: String,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<types::LogbookEntry>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let mut endpoint = format!("{}/api/logbook", read_lock.instance_url);

        match timestamp {
            Some(timestamp) => {
                let formatted_timestamp = timestamp.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                endpoint = endpoint + &formatted_timestamp.to_string();
            }
            None => {}
        };
        let mut request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        request = request.query(&[("entity", entity)]);

        match end_time {
            Some(end_time) => {
                let formatted_timestamp = end_time.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                request = request.query(&[("end_time", formatted_timestamp)]);
            }
            None => {}
        };
        let response = request.send().await?;

        let resp_json: Vec<types::LogbookEntry> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn states(self) -> Result<Vec<types::StateObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/states", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        let resp_json: Vec<types::StateObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn state_of(
        self,
        entity_id: String,
    ) -> Result<Vec<types::StateObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/states/{}", read_lock.instance_url, entity_id);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;
        let resp_json: Vec<types::StateObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn error_log(self) -> Result<String, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/error_log", read_lock.instance_url);
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let response = request.send().await?;

        let resp: String = response.text().await?;

        return Ok(resp);
    }

    pub async fn camera_proxy(self, camera_entity_id: String) -> Result<(), errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!(
            "{}/api/camera_proxy/{}",
            read_lock.instance_url, camera_entity_id
        );
        let request = reqwest::Client::new()
            .get(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);
        let _response = request.send().await?;

        return Ok(());
    }

    pub async fn state_change(
        self,
        entity_id: String,
        state_data: Option<impl serde::Serialize>,
    ) -> Result<types::StateObject, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/states/{}", read_lock.instance_url, entity_id);
        let mut request = reqwest::Client::new()
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        match state_data {
            Some(data) => {
                request = request.json(&data);
            }
            None => {}
        }

        let response = request.send().await?;

        let resp_json: types::StateObject = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn event_fire(
        self,
        event_type: String,
        event_data: Option<impl serde::Serialize>,
    ) -> Result<String, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/events/{}", read_lock.instance_url, event_type);
        let mut request = reqwest::Client::new()
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        match event_data {
            Some(data) => {
                request = request.json(&data);
            }
            None => {}
        }

        let response = request.send().await?;

        #[derive(Serialize, Deserialize, Debug)]
        struct Response {
            message: String,
        }

        let resp_json: Response = response.json().await?;

        return Ok(resp_json.message);
    }

    pub async fn service_call<T>(
        self,
        domain: String,
        service: String,
        service_data: Option<impl serde::Serialize>,
    ) -> Result<Vec<types::StateObject>, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!(
            "{}/api/services/{}/{}",
            read_lock.instance_url, domain, service
        );
        let mut request = reqwest::Client::new()
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        match service_data {
            Some(data) => {
                request = request.json(&data);
            }
            None => {}
        }

        let response = request.send().await?;

        let resp_json: Vec<types::StateObject> = response.json().await?;

        return Ok(resp_json);
    }

    pub async fn template_render(self, template: String) -> Result<String, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/template", read_lock.instance_url);

        #[derive(Serialize, Deserialize, Debug)]
        struct Template {
            template: String,
        }

        let template_struct = Template { template: template };
        let request = reqwest::Client::new()
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json");

        drop(read_lock);

        let response = request.json(&template_struct).send().await?;

        let resp: String = response.text().await?;

        return Ok(resp);
    }

    pub async fn check_config(self) -> Result<types::CheckConfig, errors::Error> {
        let mut read_lock = self.ha_client.read().unwrap();
        if read_lock.token.need_refresh() {
            drop(read_lock);
            let mut write_lock = self.ha_client.write().unwrap();
            write_lock.refresh_oauth_token().await?;
            read_lock = self.ha_client.read().unwrap();
        }

        let endpoint = format!("{}/api/config/core/check_config", read_lock.instance_url);
        let response = reqwest::Client::new()
            .post(endpoint.as_str())
            .header(
                "Authorization",
                format!("Bearer {}", read_lock.token.as_string()?),
            )
            .header("content-type", "application/json")
            .send()
            .await?;

        drop(read_lock);

        let resp_json: types::CheckConfig = response.json().await?;

        return Ok(resp_json);
    }
}

impl TryFrom<Weak<RwLock<crate::HomeAssistantAPI>>> for Rest {
    type Error = errors::Error;

    fn try_from(weak: Weak<RwLock<crate::HomeAssistantAPI>>) -> Result<Self, Self::Error> {
        match weak.upgrade() {
            Some(ptr) => Ok(Self { ha_client: ptr }),
            None => Err(errors::Error::HaApi(String::from(
                "Can't create Rest Client weak ptr returned none",
            ))),
        }
    }
}

impl From<Arc<RwLock<crate::HomeAssistantAPI>>> for Rest {
    fn from(ptr: Arc<RwLock<crate::HomeAssistantAPI>>) -> Self {
        Self { ha_client: ptr }
    }
}
