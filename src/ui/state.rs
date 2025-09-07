use std::{
    fmt::{format, Display},
    path::PathBuf,
};

use iced::widget::combo_box;
use yt_dlp::model::format::{Container, Extension};

#[derive(Debug, Clone)]
pub struct FormatListItem {
    pub format_id: String,
    pub format_note: String,
    pub container: String,
    pub audio_codec: String,
    pub audio_ext: String,
    pub video_codec: String,
    pub video_ext: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub quality: f64,
    pub file_size: u64,
}

impl FormatListItem {
    pub fn new(format: &yt_dlp::model::format::Format) -> Self {
        Self {
            format_id: format.format_id.clone(),
            format_note: format.format_note.clone().unwrap_or(String::new()),
            container: format!("{}", format.container.clone().unwrap_or(Container::Unknown))
                .replace("Container(", "")
                .replace(")", ""),
            audio_codec: format
                .codec_info
                .audio_codec
                .clone()
                .unwrap_or(String::new()),
            audio_ext: format!("{}", format.codec_info.audio_ext.clone())
                .replace("Extension(", "")
                .replace(")", ""),
            video_codec: format
                .codec_info
                .video_codec
                .clone()
                .unwrap_or(String::new()),
            video_ext: format!("{}", format.codec_info.video_ext.clone())
                .replace("Extension(", "")
                .replace(")", ""),
            width: format.video_resolution.width.unwrap_or(0),
            height: format.video_resolution.height.unwrap_or(0),
            fps: format.video_resolution.fps.unwrap_or(0.0.into()).into(),
            quality: format.quality_info.quality.unwrap_or(0.0.into()).into(),
            file_size: format.file_info.filesize.unwrap_or(0) as u64,
        }
    }
}

impl Display for FormatListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_text = format!(
            "{} ({}: {}) -",
            self.format_id, self.format_note, self.container
        );
        if self.video_codec.len() > 0 {
            display_text
                .push_str(format!(" video: {} ({})", self.video_codec, self.video_ext).as_str());
        }

        if self.audio_codec.len() > 0 {
            display_text
                .push_str(format!(" audio: {} ({})", self.audio_codec, self.audio_ext).as_str());
        }

        if self.width > 0 {
            display_text.push_str(format!(" {}x{}", self.width, self.height).as_str());
        }

        if self.fps > 0.0 {
            display_text.push_str(format!(" {}fps", self.fps).as_str());
        }

        if self.quality > 0.0 {
            display_text.push_str(format!(" Q: {}", self.quality).as_str());
        }

        if self.file_size > 0 {
            let mut refinedFileSize = self.file_size;
            let mut unit = "B";
            if refinedFileSize > 1024 {
                refinedFileSize /= 1024 ;
                unit = "KB";
            }

            if refinedFileSize > 1024 {
                refinedFileSize /= 1024 ;
                unit = "MB";
            }

            if refinedFileSize > 1024 {
                refinedFileSize /= 1024 ;
                unit = "GB";
            }

            display_text.push_str(format!(" {}{}", refinedFileSize, unit).as_str());
        }
        write!(f, "{}", display_text)
    }
}

pub struct DownloaderUIState {
    pub status_message: String,
    pub disabled: bool,
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
    pub video_tags: Vec<String>,
    pub video_categories: Vec<String>,
    pub yt_dlp_version: String,
    pub thumbnail_path: Option<PathBuf>,
    pub video_path: Option<PathBuf>,
    pub show_download_button: bool,
    pub is_video_downloading: bool,
    pub is_video_downloaded: bool,
    pub video_size: f64,
    pub downloaded_size: f64,
    pub progress: f32,
    pub video_formats: Vec<yt_dlp::model::format::Format>,
    pub format_selection_list_video: combo_box::State<FormatListItem>,
    pub format_selection_list_audio: combo_box::State<FormatListItem>,
    pub format_selection_list_audio_video: combo_box::State<FormatListItem>,
    pub selected_format_video: Option<FormatListItem>,
    pub selected_format_audio: Option<FormatListItem>,
    pub selected_format_audio_video: Option<FormatListItem>,
}

impl Default for DownloaderUIState {
    fn default() -> Self {
        Self {
            status_message: String::from("Ready"),
            disabled: false,
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
            video_size: 0.0,
            downloaded_size: 0.0,
            progress: 0.0,
            format_selection_list_video: combo_box::State::new(Vec::new()),
            format_selection_list_audio: combo_box::State::new(Vec::new()),
            format_selection_list_audio_video: combo_box::State::new(Vec::new()),
            selected_format_video: None,
            selected_format_audio: None,
            selected_format_audio_video: None,
        }
    }
}
