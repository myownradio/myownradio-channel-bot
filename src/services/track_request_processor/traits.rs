use crate::services::track_request_processor::types::{
    RequestId, TrackFetcherContext, TrackFetcherState,
};
use crate::types::{
    AudioMetadata, DownloadId, RadioManagerChannelId, RadioManagerLinkId, RadioManagerTrackId,
    TopicId, UserId,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub(crate) enum StateStorageError {
    #[error("Unable to deserialize object")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Object already exists")]
    ObjectExists,
    #[error("Object has not been found")]
    ObjectNotFound,
    #[error("Unexpected error")]
    SerdeError(),
    #[error("Unexpected error")]
    Unexpected,
}

#[async_trait]
pub(crate) trait StateStorage {
    async fn create_state(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
        state: TrackFetcherState,
    ) -> Result<(), StateStorageError>;
    async fn create_context(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
        state: TrackFetcherContext,
    ) -> Result<(), StateStorageError>;
    async fn update_state(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
        state: &TrackFetcherState,
    ) -> Result<(), StateStorageError>;
    async fn load_state(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
    ) -> Result<TrackFetcherState, StateStorageError>;
    async fn load_context(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
    ) -> Result<TrackFetcherContext, StateStorageError>;
    async fn delete_state(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
    ) -> Result<(), StateStorageError>;
    async fn delete_context(
        &self,
        user_id: &UserId,
        request_id: &RequestId,
    ) -> Result<(), StateStorageError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum SearchProviderError {
    #[error("Unexpected error")]
    Unexpected,
}

pub(crate) struct SearchResult {
    pub(crate) topic_id: TopicId,
    pub(crate) title: String,
}

#[async_trait]
pub(crate) trait SearchProvider {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>, SearchProviderError>;
    async fn get_url(&self, topic_id: &TopicId) -> Result<Option<Vec<u8>>, SearchProviderError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum DownloaderError {
    #[error("Unexpected error")]
    Unexpected,
}

pub(crate) enum DownloadingStatus {
    Downloading,
    Complete,
}

pub(crate) struct DownloadingEntry {
    pub(crate) status: DownloadingStatus,
    pub(crate) files: Vec<String>,
}

#[async_trait]
pub(crate) trait Downloader {
    async fn create(
        &self,
        path_to_download: &str,
        url: Vec<u8>,
    ) -> Result<DownloadId, DownloaderError>;
    async fn get(
        &self,
        download_id: &DownloadId,
    ) -> Result<Option<DownloadingEntry>, DownloaderError>;
    async fn delete(&self, download_id: &DownloadId) -> Result<(), DownloaderError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum MetadataServiceError {
    #[error("Unexpected error")]
    Unexpected,
}

#[async_trait]
pub(crate) trait MetadataService {
    async fn get_audio_metadata(
        &self,
        file_path: &str,
    ) -> Result<Option<AudioMetadata>, MetadataServiceError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RadioManagerError {
    #[error("Unexpected error")]
    Unexpected,
}

#[async_trait]
pub(crate) trait RadioManager {
    async fn upload_audio_track(
        &self,
        user_id: &UserId,
        path_to_audio_file: &str,
    ) -> Result<RadioManagerTrackId, RadioManagerError>;
    async fn add_track_to_channel_playlist(
        &self,
        user_id: &UserId,
        track_id: &RadioManagerTrackId,
        channel_id: &RadioManagerChannelId,
    ) -> Result<RadioManagerLinkId, RadioManagerError>;
}