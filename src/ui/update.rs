use crate::ui::state::FormatListItem;

use super::{message::Message as UIMessage, state::DownloaderUIState};
use iced::{widget::combo_box, Task};

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
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::InstallLibraries);

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
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::UpdateLibraries);
            Task::none()
        }
        UIMessage::LibrariesUpdated => {
            downloader_ui_state.status_message = "Libraries updated.".to_string();
            downloader_ui_state.disabled = false;
            Task::none()
        }
        UIMessage::UrlChanged(url) => {
            downloader_ui_state.video_url = url.clone();
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::UrlChanged(url));
            downloader_ui_state.disabled = false;
            downloader_ui_state.is_video_downloaded = false;
            downloader_ui_state.is_video_downloading = false;
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
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::FetchInfo);
            Task::none()
        }
        UIMessage::InfoFetched(video_info) => {
            downloader_ui_state.status_message = "Video Info Fetched.".to_string();
            downloader_ui_state.disabled = false;

            let auto_selected_video_format = video_info.best_video_format().unwrap().clone();
            let auto_selected_audio_format = video_info.best_audio_format().unwrap().clone();

            let mut auto_selected_video_format_id: String = String::new();
            let mut auto_selected_video_format_match_level: i32 = 0;
            let mut auto_selected_audio_format_id: String = String::new();
            let mut auto_selected_audio_format_match_level: i32 = 0;

            for video_format in &video_info.formats {
                if video_format.is_video() {
                    let mut video_format_match_level = 0;
                    if video_format.format_id == auto_selected_video_format.format_id {
                        auto_selected_video_format_id = video_format.format_id.clone();
                        video_format_match_level = 1;
                    }
                    if video_format_match_level > auto_selected_video_format_match_level {
                        auto_selected_video_format_match_level = video_format_match_level;
                        break;
                    }
                }
            }

            for audio_format in &video_info.formats {
                if audio_format.is_audio() {
                    let mut audio_format_match_level = 0;
                    if audio_format.quality_info.quality
                        == auto_selected_audio_format.quality_info.quality
                    {
                        audio_format_match_level += 1;
                    }
                    if audio_format.codec_info.audio_ext
                        == auto_selected_audio_format.codec_info.audio_ext
                    {
                        audio_format_match_level += 1;
                    }
                    if audio_format
                        .format_note
                        .as_ref()
                        .unwrap()
                        .contains("original")
                    {
                        audio_format_match_level += 2;
                    }
                    if audio_format.codec_info.audio_codec.clone().unwrap()
                        == auto_selected_audio_format
                            .codec_info
                            .audio_codec
                            .clone()
                            .unwrap()
                    {
                        audio_format_match_level += 1;
                    }

                    if audio_format.format_id == auto_selected_audio_format.format_id {
                        audio_format_match_level += 1;
                    }

                    if audio_format_match_level > auto_selected_audio_format_match_level {
                        auto_selected_audio_format_id = audio_format.format_id.clone();
                        auto_selected_audio_format_match_level = audio_format_match_level;
                    }
                }
            }

            downloader_ui_state.video_id = video_info.id;
            downloader_ui_state.video_title = video_info.title;
            downloader_ui_state.video_channel = video_info.channel;
            downloader_ui_state.video_channel_id = video_info.channel_id;
            downloader_ui_state.video_description = video_info.description;

            downloader_ui_state.video_formats = video_info.formats;

            downloader_ui_state.format_selection_list_video = combo_box::State::new(
                downloader_ui_state
                    .video_formats
                    .clone()
                    .into_iter()
                    .filter(|format| format.format_note.as_ref().unwrap() != "storyboard")
                    .filter(|format| format.is_video())
                    .map(|format| FormatListItem::new(&format))
                    .collect(),
            );
            downloader_ui_state.format_selection_list_audio = combo_box::State::new(
                downloader_ui_state
                    .video_formats
                    .clone()
                    .into_iter()
                    .filter(|format| format.format_note.as_ref().unwrap() != "storyboard")
                    .filter(|format| format.is_audio())
                    .map(|format| FormatListItem::new(&format))
                    .collect(),
            );

            for format_selection_list_video_item in
                downloader_ui_state.format_selection_list_video.options()
            {
                if format_selection_list_video_item.format_id == auto_selected_video_format_id {
                    downloader_ui_state.selected_format_video =
                        Some(format_selection_list_video_item.clone());
                    let _ = downloader_ui_state.sender.as_ref().unwrap().send(
                        UIMessage::SelectVideoFormat(format_selection_list_video_item.clone()),
                    );
                    break;
                }
            }

            for format_selection_list_audio_item in
                downloader_ui_state.format_selection_list_audio.options()
            {
                if format_selection_list_audio_item.format_id == auto_selected_audio_format_id {
                    downloader_ui_state.selected_format_audio =
                        Some(format_selection_list_audio_item.clone());
                    let _ = downloader_ui_state.sender.as_ref().unwrap().send(
                        UIMessage::SelectAudioFormat(format_selection_list_audio_item.clone()),
                    );
                    break;
                }
            }

            Task::done(UIMessage::FetchThumbnail)
        }
        UIMessage::FetchThumbnail => {
            downloader_ui_state.status_message = "Fetching Video Thumbnail...".to_string();
            downloader_ui_state.disabled = true;
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::FetchThumbnail);
            Task::none()
        }
        UIMessage::ThumbnailFetched(thumbnail_path) => {
            downloader_ui_state.status_message = "Video Thumbnail Fetched.".to_string();
            downloader_ui_state.disabled = false;

            downloader_ui_state.thumbnail_path = thumbnail_path;
            downloader_ui_state.show_download_button = true;
            Task::none()
        }
        UIMessage::SelectAudioFormat(format) => {
            downloader_ui_state.selected_format_audio = Some(format.clone());
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::SelectAudioFormat(format));
            Task::none()
        }
        UIMessage::SelectVideoFormat(format) => {
            downloader_ui_state.selected_format_video = Some(format.clone());
            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::SelectVideoFormat(format));
            Task::none()
        }
        UIMessage::DownloadVideo => {
            downloader_ui_state.is_video_downloading = true;
            downloader_ui_state.status_message = "Downloading Video...".to_string();
            downloader_ui_state.disabled = true;

            let _ = downloader_ui_state
                .sender
                .as_ref()
                .unwrap()
                .send(UIMessage::DownloadVideo);

            Task::none()
        }
        UIMessage::VideoDownloaded(video_path) => {
            downloader_ui_state.status_message = format!(
                "Video Downloaded to {}.",
                video_path.clone().unwrap().display()
            );
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
