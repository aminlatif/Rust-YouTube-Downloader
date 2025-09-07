use super::super::downloader::VideoDownloader;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc::{ UnboundedReceiver, UnboundedSender}, RwLock};

pub struct DownloaderUIState {
    pub video_downloader: Arc<RwLock<VideoDownloader>>,
    pub reciever: Option<tokio::sync::broadcast::Receiver<super::message::Message>>,
    pub sender: Option<tokio::sync::broadcast::Sender<super::message::Message>>,
    pub executables_dir: String,
    pub output_dir: String,
    pub video_url: String,
    pub video_id: String,
    pub video_title: String,
    pub video_description: String,
    pub video_channel: String,
    pub video_channel_id: String,
    pub video_formats: Vec<String>,
    pub video_tags: Vec<String>,
    pub video_categories: Vec<String>,
    pub yt_dlp_version: String,
    pub thumbnail_path: Option<PathBuf>,
    pub video_path: Option<PathBuf>,
    pub show_download_button: bool,
    pub is_video_downloading: bool,
    pub is_video_downloaded: bool,
}

impl Default for DownloaderUIState {
    fn default() -> Self {
        Self {
            video_downloader: Arc::new(RwLock::new(VideoDownloader::default())),
            reciever: None,
            sender: None,
            executables_dir: String::from("libs"),
            output_dir: String::from("output"),
            video_url: String::from("https://www.youtube.com/watch?v=1A6uPztchXk"),
            video_id: String::new(),
            video_title: String::new(),
            video_description: String::new(),
            video_channel: String::new(),
            video_channel_id: String::new(),
            video_formats: Vec::new(),
            video_tags: Vec::new(),
            video_categories: Vec::new(),
            yt_dlp_version: String::new(),
            thumbnail_path: None,
            video_path: None,
            show_download_button: false,
            is_video_downloading: false,
            is_video_downloaded: false,
        }
    }
}
