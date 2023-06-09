use crate::services::torrent_parser::{get_files_count, TorrentParserError};
use async_lock::Mutex;
use base64::{engine::general_purpose::STANDARD, Engine};
use transmission_rpc::types::{
    BasicAuth, Id, RpcResponse, Torrent, TorrentAction, TorrentAddArgs, TorrentAddedOrDuplicate,
    TorrentSetArgs,
};
use transmission_rpc::TransClient;

pub(crate) struct TransmissionClient {
    client: Mutex<TransClient>,
    download_dir: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum TransmissionClientError {
    #[error("Torrent not found")]
    NotFound,
    #[error("Erroneous result: {0}")]
    ErroneousResult(String),
    #[error("Unable to perform RPC request on transmission server: {0}")]
    TransmissionError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Torrent file parsing error: {0}")]
    TorrentParserError(#[from] TorrentParserError),
}

pub(crate) type Result<T> = std::result::Result<T, TransmissionClientError>;

impl TransmissionClient {
    pub(crate) fn create(
        url: String,
        username: Option<String>,
        password: Option<String>,
        download_dir: String,
    ) -> Self {
        let url = (&url).parse().unwrap();
        let client = match (username, password) {
            (Some(user), Some(password)) => {
                TransClient::with_auth(url, BasicAuth { user, password })
            }
            _ => TransClient::new(url),
        };

        Self {
            client: Mutex::new(client),
            download_dir,
        }
    }

    pub(crate) async fn select_files(&self, torrent_id: &i64, file_indexes: &[i32]) -> Result<()> {
        let id = Id::Id(*torrent_id);

        self.client
            .lock()
            .await
            .torrent_set(
                TorrentSetArgs {
                    files_wanted: Some(file_indexes.iter().map(|i| *i).collect()),
                    ..TorrentSetArgs::default()
                },
                Some(vec![id.clone()]),
            )
            .await?;

        self.client
            .lock()
            .await
            .torrent_action(TorrentAction::Start, vec![id])
            .await?;

        Ok(())
    }

    pub(crate) async fn add(&self, torrent_file_content: Vec<u8>) -> Result<i64> {
        let files_count = get_files_count(&torrent_file_content)?;
        let metainfo = STANDARD.encode(torrent_file_content);

        let RpcResponse { arguments, result } = self
            .client
            .lock()
            .await
            .torrent_add(TorrentAddArgs {
                metainfo: Some(metainfo.clone()),
                download_dir: Some(self.download_dir.clone()),
                // Initialize new torrent with disabling download of any files.
                files_unwanted: Some((0..files_count as i32).collect()),
                ..TorrentAddArgs::default()
            })
            .await?;

        if result != "success" {
            return Err(TransmissionClientError::ErroneousResult(result));
        }

        let torrent = match arguments {
            TorrentAddedOrDuplicate::TorrentAdded(torrent) => torrent,
            TorrentAddedOrDuplicate::TorrentDuplicate(torrent) => torrent,
        };

        Ok(torrent.id.unwrap())
    }

    pub(crate) async fn remove(&self, torrent_id: &i64) -> Result<()> {
        let RpcResponse { result, .. } = self
            .client
            .lock()
            .await
            .torrent_remove(vec![Id::Id(*torrent_id)], false)
            .await?;

        if result != "success" {
            return Err(TransmissionClientError::ErroneousResult(result));
        }

        Ok(())
    }

    pub(crate) async fn remove_with_data(&self, torrent_id: &i64) -> Result<()> {
        let id = Id::Id(*torrent_id);
        let RpcResponse { result, .. } = self
            .client
            .lock()
            .await
            .torrent_remove(vec![id], true)
            .await?;

        if result != "success" {
            return Err(TransmissionClientError::ErroneousResult(result));
        }

        Ok(())
    }

    pub(crate) async fn get(&self, torrent_id: &i64) -> Result<Torrent> {
        let RpcResponse { result, arguments } = self
            .client
            .lock()
            .await
            .torrent_get(None, Some(vec![Id::Id(*torrent_id)]))
            .await?;

        if result != "success" {
            return Err(TransmissionClientError::ErroneousResult(result));
        }

        let maybe_torrent = arguments.torrents.into_iter().next();

        maybe_torrent.ok_or(TransmissionClientError::NotFound)
    }

    pub(crate) async fn check_connection(&self) -> Result<()> {
        let RpcResponse {
            result,
            arguments: _,
        } = self.client.lock().await.torrent_get(None, None).await?;

        if result != "success" {
            return Err(TransmissionClientError::ErroneousResult(result));
        }

        Ok(())
    }
}
