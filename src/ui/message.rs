use iced::Size;
use std::path::PathBuf;
use yt_dlp::model::Video;

use crate::ui::state::FormatListItem;

#[derive(Debug, Clone)]
pub enum Message {
    Number(i32),
    StatusMessage(String),
    Disabled(bool),
    InstallLibraries,
    LibrariesInstalled,
    UpdateLibraries,
    LibrariesUpdated,
    UIUpdated,
    UrlChanged(String),
    FetchInfo,
    InfoFetched(Video),
    FetchThumbnail,
    ThumbnailFetched(Option<PathBuf>),
    DownloadVideo,
    VideoDownloaded(Option<PathBuf>),
    ProgressUpdated(f64, f32),
    SelectAudioFormat(FormatListItem),
    SelectVideoFormat(FormatListItem),
}
