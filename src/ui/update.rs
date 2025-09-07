use super::{message::Message as UIMessage, state::DownloaderUIState};
use iced::Task;

pub fn update(downloader_ui_state: &mut DownloaderUIState, message: UIMessage) -> Task<UIMessage> {
    match message {
        UIMessage::StatusMessage(msg) => {
            downloader_ui_state.status_message = msg;
            Task::none()
        }
        UIMessage::Disabled(disabled) => {
            downloader_ui_state.disabled = disabled;
            Task::none()
        }
        UIMessage::InstallLibraries => {
            downloader_ui_state.status_message = "Installing libraries...".to_string();
            downloader_ui_state.disabled = true;
            let _ = downloader_ui_state.sender.as_ref().unwrap().send(UIMessage::InstallLibraries);

            Task::none()
        }
        UIMessage::LibrariesInstalled => {
            downloader_ui_state.status_message = "Libraries installed.".to_string();
            downloader_ui_state.disabled = false;
            Task::none()
        }
        UIMessage::UpdateLibraries => {
            downloader_ui_state.status_message = "Updating libraries...".to_string();
            downloader_ui_state.disabled = true;
            let _ = downloader_ui_state.sender.as_ref().unwrap().send(UIMessage::UpdateLibraries);
            Task::none()
        }
        UIMessage::LibrariesUpdated => {
            downloader_ui_state.status_message = "Libraries updated.".to_string();
            downloader_ui_state.disabled = false;
            Task::none()
        }
        UIMessage::UrlChanged(url) => {
            downloader_ui_state.video_url = url.clone();
            let _ =downloader_ui_state.sender.as_ref().unwrap().send(UIMessage::UrlChanged(url));
            Task::none()
        }
        UIMessage::FetchInfo => {
            downloader_ui_state.status_message = "Fetching Video Info...".to_string();
            downloader_ui_state.disabled = true;

            downloader_ui_state.video_id = String::new();
            downloader_ui_state.video_title = String::new();
            downloader_ui_state.video_channel = String::new();
            downloader_ui_state.video_channel_id = String::new();
            downloader_ui_state.video_description = String::new();
            downloader_ui_state.thumbnail_path = None;
            let _ = downloader_ui_state.sender.as_ref().unwrap().send(UIMessage::FetchInfo);
            Task::none()
        }
        UIMessage::InfoFetched(video_info) => {
            downloader_ui_state.status_message = "Video Info Fetched.".to_string();
            downloader_ui_state.disabled = false;

            downloader_ui_state.video_id = video_info.id;
            downloader_ui_state.video_title = video_info.title;
            downloader_ui_state.video_channel = video_info.channel;
            downloader_ui_state.video_channel_id = video_info.channel_id;
            downloader_ui_state.video_description = video_info.description;

            Task::done(UIMessage::FetchThumbnail)
        }
        UIMessage::FetchThumbnail => {
            downloader_ui_state.status_message = "Fetching Video Thumbnail...".to_string();
            downloader_ui_state.disabled = true;
            let _ = downloader_ui_state.sender.as_ref().unwrap().send(UIMessage::FetchThumbnail);
            Task::none()
        }
        UIMessage::ThumbnailFetched(thumbnail_path) => {
            downloader_ui_state.status_message = "Video Thumbnail Fetched.".to_string();
            downloader_ui_state.disabled = false;

            downloader_ui_state.thumbnail_path = thumbnail_path;
            downloader_ui_state.show_download_button = true;
            Task::none()
        }
        UIMessage::DownloadVideo => {
            downloader_ui_state.is_video_downloading = true;
            downloader_ui_state.status_message = "Downloading Video...".to_string();
            downloader_ui_state.disabled = true;

            let _ = downloader_ui_state.sender.as_ref().unwrap().send(UIMessage::DownloadVideo);

            Task::none()
        }
        UIMessage::VideoDownloaded(video_path) => {
            downloader_ui_state.status_message = format!("Video Downloaded to {}.", video_path.clone().unwrap().display());
            downloader_ui_state.disabled = false;

            downloader_ui_state.video_path = video_path;
            downloader_ui_state.is_video_downloading = false;
            downloader_ui_state.is_video_downloaded = true;

            Task::none()
        }
        UIMessage::ProgressUpdated(downloaded_size, progress) => {
            downloader_ui_state.downloaded_size = downloaded_size;
            downloader_ui_state.progress = progress;
            Task::none()
        }
        UIMessage::Number(num) => {
            println!("UI Recieved number: {}", num);
            Task::none()
        }
        _ => Task::none(),
    }
}
