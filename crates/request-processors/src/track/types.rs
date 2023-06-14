use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestId(Uuid);

impl Deref for RequestId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<RequestId> for Uuid {
    fn into(self) -> RequestId {
        RequestId(self)
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct RadioManagerTrackId(pub(crate) u64);

impl std::fmt::Display for RadioManagerTrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct RadioManagerChannelId(pub(crate) u64);

impl std::fmt::Display for RadioManagerChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct RadioManagerLinkId(pub(crate) String);

impl std::fmt::Display for RadioManagerLinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct AudioMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TorrentStatus {
    Downloading,
    Complete,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Torrent {
    pub(crate) status: TorrentStatus,
    pub(crate) files: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TopicData {
    pub(crate) topic_id: TopicId,
    pub(crate) download_id: DownloadId,
    pub(crate) title: String,
}

// UserId
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct UserId(pub(crate) u64);

impl Deref for UserId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<UserId> for u64 {
    fn into(self) -> UserId {
        UserId(self)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TopicId
#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct TopicId(pub(crate) u64);

impl Deref for TopicId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TopicId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// DownloadId
#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct DownloadId(pub(crate) u64);

impl Deref for DownloadId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TorrentId
#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct TorrentId(pub(crate) i64);

impl Deref for TorrentId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<TorrentId> for i64 {
    fn into(self) -> TorrentId {
        TorrentId(self)
    }
}

impl std::fmt::Display for TorrentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
