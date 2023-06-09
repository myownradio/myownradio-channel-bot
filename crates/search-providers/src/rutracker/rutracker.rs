use crate::rutracker::parser::{
    parse_and_validate_auth_state, parse_search_results, AuthError, ParseError,
};
use crate::TopicData;
use reqwest::redirect::Policy;
use reqwest::{Client, StatusCode};
use serde::Serialize;

const RU_TRACKER_HOST: &str = "https://rutracker.net";
const MAGIC_LOGIN_WORD: &str = "вход";

#[derive(Debug, thiserror::Error)]
pub enum RuTrackerClientError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error("Unexpected response status: {0}")]
    BadStatus(StatusCode),
}

pub struct RuTrackerClient {
    client: Client,
}

impl RuTrackerClient {
    pub async fn create(username: &str, password: &str) -> Result<Self, RuTrackerClientError> {
        let client = Client::builder()
            .redirect(Policy::limited(10))
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP Client");

        #[derive(Serialize)]
        struct LoginForm {
            login_username: String,
            login_password: String,
            login: String,
        }

        let form = LoginForm {
            login_username: username.to_string(),
            login_password: password.to_string(),
            login: MAGIC_LOGIN_WORD.to_string(),
        };

        let response = client
            .post(format!("{}/forum/login.php", RU_TRACKER_HOST))
            .form(&form)
            .send()
            .await?;

        let raw_html = response.text().await?;

        parse_and_validate_auth_state(&raw_html)?;

        Ok(Self { client })
    }

    pub async fn search_music(
        &self,
        query_str: &str,
    ) -> Result<Vec<TopicData>, RuTrackerClientError> {
        #[derive(Serialize)]
        struct Query {
            nm: String,
        }

        let query = Query {
            nm: query_str.to_string(),
        };

        let response = self
            .client
            .get(format!("{}/forum/tracker.php", RU_TRACKER_HOST))
            .query(&query)
            .send()
            .await?;

        let raw_html = response.text().await?;

        parse_and_validate_auth_state(&raw_html)?;

        Ok(parse_search_results(&raw_html)?)
    }

    pub async fn download_torrent(
        &self,
        download_id: u64,
    ) -> Result<Vec<u8>, RuTrackerClientError> {
        let response = self
            .client
            .get(format!(
                "{}/forum/dl.php?t={}",
                RU_TRACKER_HOST, download_id
            ))
            .send()
            .await?;
        let status = response.status();

        if status != StatusCode::OK {
            let raw_html = response.text().await?;
            parse_and_validate_auth_state(&raw_html)?;
            return Err(RuTrackerClientError::BadStatus(status));
        }

        Ok(response.bytes().await?.to_vec())
    }

    pub async fn check_connection(&self) -> Result<(), RuTrackerClientError> {
        let response = self
            .client
            .get(format!("{}", RU_TRACKER_HOST))
            .send()
            .await?;
        let status = response.status();

        if status != StatusCode::OK {
            return Err(RuTrackerClientError::BadStatus(status));
        }

        let raw_html = response.text().await?;
        parse_and_validate_auth_state(&raw_html)?;

        Ok(())
    }
}
