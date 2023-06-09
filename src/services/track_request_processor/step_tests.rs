use super::track_request_processor::{
    DownloadId, RadioManagerLinkId, RadioManagerTrackId, TorrentId, TrackRequestProcessingState,
    TrackRequestProcessingStep,
};
use crate::services::track_request_processor::{TopicData, TopicId};

#[test]
fn should_return_search_audio_album_by_default() {
    let state = TrackRequestProcessingState::default();

    assert_eq!(
        state.get_step(),
        TrackRequestProcessingStep::GetTopicsIntoQueue
    )
}

#[test]
fn should_return_get_album_url_if_current_topic_id_is_set() {
    let state = TrackRequestProcessingState {
        topics_queue: Some(vec![TopicData {
            topic_id: TopicId(1),
            download_id: DownloadId(1),
            title: "Title".into(),
        }]),
        ..TrackRequestProcessingState::default()
    };

    assert_eq!(
        state.get_step(),
        TrackRequestProcessingStep::DownloadNextTorrentFile
    )
}

#[test]
fn should_return_download_album_if_current_url_is_set() {
    let state = TrackRequestProcessingState {
        topics_queue: Some(vec![TopicData {
            topic_id: TopicId(1),
            download_id: DownloadId(1),
            title: "Title".into(),
        }]),
        current_torrent_data: Some(vec![]),
        ..TrackRequestProcessingState::default()
    };

    assert_eq!(state.get_step(), TrackRequestProcessingStep::Download)
}

#[test]
fn should_return_check_download_status_if_current_download_id_is_set() {
    let state = TrackRequestProcessingState {
        topics_queue: Some(vec![TopicData {
            topic_id: TopicId(1),
            download_id: DownloadId(1),
            title: "Title".into(),
        }]),
        current_torrent_data: Some(vec![]),
        current_torrent_id: Some(TorrentId(1)),
        ..TrackRequestProcessingState::default()
    };

    assert_eq!(
        state.get_step(),
        TrackRequestProcessingStep::CheckDownloadStatus
    )
}

#[test]
fn should_return_upload_to_radioterio_if_path_to_downloaded_file_is_set() {
    let state = TrackRequestProcessingState {
        topics_queue: Some(vec![TopicData {
            topic_id: TopicId(1),
            download_id: DownloadId(1),
            title: "Title".into(),
        }]),
        current_torrent_data: Some(vec![]),
        current_torrent_id: Some(TorrentId(1)),
        path_to_downloaded_file: Some("path/to/file".into()),
        ..TrackRequestProcessingState::default()
    };

    assert_eq!(
        state.get_step(),
        TrackRequestProcessingStep::UploadToRadioManager
    )
}

#[test]
fn should_return_add_track_to_radioterio_channel_if_radioterio_track_id_is_set() {
    let state = TrackRequestProcessingState {
        topics_queue: Some(vec![TopicData {
            topic_id: TopicId(1),
            download_id: DownloadId(1),
            title: "Title".into(),
        }]),
        current_torrent_data: Some(vec![]),
        current_torrent_id: Some(TorrentId(1)),
        path_to_downloaded_file: Some("path/to/file".into()),
        radio_manager_track_id: Some(RadioManagerTrackId(1)),
        ..TrackRequestProcessingState::default()
    };

    assert_eq!(
        state.get_step(),
        TrackRequestProcessingStep::AddToRadioManagerChannel
    )
}

#[test]
fn should_return_finish_if_radioterio_link_id_is_set() {
    let state = TrackRequestProcessingState {
        topics_queue: Some(vec![TopicData {
            topic_id: TopicId(1),
            download_id: DownloadId(1),
            title: "Title".into(),
        }]),
        current_torrent_data: Some(vec![]),
        current_torrent_id: Some(TorrentId(1)),
        path_to_downloaded_file: Some("path/to/file".into()),
        radio_manager_track_id: Some(RadioManagerTrackId(1)),
        radio_manager_link_id: Some(RadioManagerLinkId("foo".into())),
        ..TrackRequestProcessingState::default()
    };

    assert_eq!(state.get_step(), TrackRequestProcessingStep::Finish)
}
