use iced::Size;
use std::path::PathBuf;
use yt_dlp::model::Video;

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    Number(i32),
    WindowResized(Size),
    UIUpdated,
    UrlChanged(String),
    FetchInfo,
    InfoFetched(Video),
    FetchThumbnail,
    ThumbnailFetched(Option<PathBuf>),
    DownloadVideo,
    VideoDownloaded(Option<PathBuf>),
}
