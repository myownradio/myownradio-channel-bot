use crate::services::track_request_processor::AudioMetadata;
use reqwest::Client;

const OPENAI_ENDPOINT: &str = "https://api.openai.com";

pub(crate) struct OpenAIService {
    openai_api_key: String,
    client: Client,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum OpenAIServiceError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl OpenAIService {
    pub(crate) fn create(openai_api_key: String) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP Client");

        Self {
            openai_api_key,
            client,
        }
    }

    pub(crate) async fn get_audio_tracks_suggestion(
        &self,
        tracks_list: &Vec<AudioMetadata>,
    ) -> Result<Vec<AudioMetadata>, OpenAIServiceError> {
        let tracks_list_str = tracks_list
            .iter()
            .map(|m| format!("{} - {}", m.artist, m.title))
            .collect::<Vec<_>>()
            .join("\n");

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", OPENAI_ENDPOINT))
            .header("Authorization", format!("Bearer {}", self.openai_api_key))
            .json(&serde_json::json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {"role": "system", "content": "Here are the rules you should follow:\n\n1. The user will provide you with a list of audio tracks, where each track is separated by a new line.\n\n2. Create a valid JSON array containing two audio tracks that will ideally fit existing ones in the list in terms of vibe and mood. Objects should have the following fields: title, artist and album.\n\n3. Without any additional comments and descriptions. Just array."},
                    {"role": "user", "content": tracks_list_str}
                ]
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        let response_content = response
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .and_then(|str| serde_json::from_str::<Vec<AudioMetadata>>(str).ok())
            .unwrap_or_default();

        Ok(response_content)
    }
}
