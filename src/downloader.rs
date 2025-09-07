use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, error, info};
use super::ui::message::Message as UIMessage;
use yt_dlp::model::ExtractorInfo;
use yt_dlp::model::Version;
use yt_dlp::model::Video;
use yt_dlp::{fetcher::deps::Libraries, Youtube};

use crate::ui::message::Message;

#[derive(Debug)]
pub struct VideoDownloader {
    pub executables_dir: PathBuf,
    pub output_dir: PathBuf,
    pub video_url: String,
    yt_dlp_executable_path: PathBuf,
    ffmpeg_executable_path: PathBuf,
    libraries: Libraries,
    fetcher: Youtube,
    pub output_file_name: String,
    pub video_info: Video,
    pub thumbnail_path: Option<PathBuf>,
    pub video_path: Option<PathBuf>,
}

impl Default for VideoDownloader {
    fn default() -> Self {
        Self::new(
            "libs",
            "output",
            "https://www.youtube.com/watch?v=1A6uPztchXk",
        )
    }
}

impl VideoDownloader {
    pub fn new(executables_dir: &str, output_dir: &str, video_url: &str) -> Self {
        let executables_dir_path_buf = PathBuf::from(executables_dir);
        let output_dir_path_buf = PathBuf::from(output_dir);
        let yt_dlp_executable_path_buf = executables_dir_path_buf.join("yt-dlp");
        let ffmpeg_executable_path_buf = executables_dir_path_buf.join("ffmpeg");
        let libraries = Libraries::new(
            yt_dlp_executable_path_buf.clone(),
            ffmpeg_executable_path_buf.clone(),
        );
        let fetcher = Youtube::new(libraries.clone(), output_dir_path_buf.clone()).unwrap();
        Self {
            executables_dir: executables_dir_path_buf,
            output_dir: output_dir_path_buf,
            video_url: video_url.to_string(),
            yt_dlp_executable_path: yt_dlp_executable_path_buf,
            ffmpeg_executable_path: ffmpeg_executable_path_buf,
            libraries: libraries,
            fetcher: fetcher,
            output_file_name: String::new(),
            video_info: Video {
                id: String::new(),
                title: String::new(),
                channel: String::new(),
                channel_id: String::new(),
                description: String::new(),
                formats: Vec::new(),
                automatic_captions: HashMap::new(),
                thumbnail: String::new(),
                availability: String::new(),
                upload_date: 0,
                view_count: 0,
                like_count: Some(0),
                comment_count: Some(0),
                channel_url: String::new(),
                channel_follower_count: Some(0),
                thumbnails: Vec::new(),
                tags: Vec::new(),
                categories: Vec::new(),
                age_limit: 0,
                has_drm: None,
                live_status: String::new(),
                playable_in_embed: true,
                extractor_info: ExtractorInfo {
                    extractor: String::new(),
                    extractor_key: String::new(),
                },
                version: Version {
                    version: String::new(),
                    current_git_head: Some(String::new()),
                    release_git_head: Some(String::new()),
                    repository: String::new(),
                },
            },
            thumbnail_path: None,
            video_path: None,
        }
    }
}

pub fn change_video_url(video_downloader: &mut VideoDownloader, video_url: String) {
    video_downloader.video_url = video_url;
}

pub async fn get_video_info(video_downloader: &mut VideoDownloader) -> anyhow::Result<Video> {
    match video_downloader
        .fetcher
        .fetch_video_infos(video_downloader.video_url.clone())
        .await
    {
        Ok(video_info) => {
            debug!("Video infos recieved.");
            video_downloader.video_info = video_info;
            video_downloader.output_file_name =
                sanitize_filename(video_downloader.video_info.title.clone().as_str())
                    .trim()
                    .to_string();
            let info_file_name = video_downloader.output_file_name.clone() + ".txt";
            let info_file_path = "output/".to_string() + &info_file_name;
            fs::write(
                info_file_path,
                format!("{:#?}", video_downloader.video_info),
            )
            .unwrap();
            Ok(video_downloader.video_info.clone())
        }
        Err(e) => {
            error!("Error fetching video infos: {}", e);
            Err(e.into())
        }
    }
}

pub async fn get_video_thumbnail(
    video_downloader: &mut VideoDownloader,
) -> anyhow::Result<PathBuf> {
    let thumbnail_extension = video_downloader
        .video_info
        .thumbnail
        .split(".")
        .last()
        .unwrap();
    match video_downloader
        .fetcher
        .download_thumbnail_from_url(
            video_downloader.video_url.clone(),
            video_downloader.output_file_name.clone() + "." + thumbnail_extension,
        )
        .await
    {
        Ok(path) => {
            video_downloader.thumbnail_path = Some(path);
            Ok(video_downloader.thumbnail_path.clone().unwrap())
        }
        Err(e) => {
            error!("Error downloading thumbnail: {}", e);
            Err(e.into())
        }
    }
}

pub async fn download_video(
    video_downloader: &mut VideoDownloader,
    tx: &tokio::sync::broadcast::Sender<Message>,
) -> anyhow::Result<PathBuf> {
    debug!("Downloading video...");
    let video_info = video_downloader.video_info.clone();
    let video_path = video_downloader.output_file_name.clone() + ".mp4";
    let video_fetcher = video_downloader.fetcher.clone();
    let tx_clone = tx.clone();
    video_downloader.video_path = Some(video_path.clone().into());

    debug!("Starting Download...");

    let download_id = video_fetcher
        .download_video_with_progress(&video_info, video_path, move |downloaded, total| {
            let percentage = if total > 0 {
                (downloaded as f64 / total as f64 * 100.0) as u64
            } else {
                0
            };
            let _ =tx_clone.send(UIMessage::ProgressUpdated(downloaded as f64, percentage as f32));
            info!("Progress: {}/{} bytes ({}%)", downloaded, total, percentage);
        })
        .await
        .expect("Failed to download video");

    debug!("download id: {}", download_id);

    video_fetcher
        .wait_for_download(download_id)
        .await
        .expect("Waiting for download to finish failed");
    debug!("Download finished");

    debug!("removing temp files...");
    for entry in fs::read_dir(video_downloader.output_dir.clone())? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            if entry.file_name().to_str().unwrap().contains("temp_audio_")
                || entry.file_name().to_str().unwrap().contains("temp_video_")
            {
                fs::remove_file(entry.path())?;
            }
        }
    }

    debug!("Downloading captions...");
    for caption_group in &video_downloader.video_info.automatic_captions {
        let caption_languages = caption_group.0;
        if caption_languages.contains("en")
            || caption_languages.contains("orig")
            || caption_languages.contains("fa")
        {
            for caption in caption_group.1 {
                let caption_extension = &caption.extension;
                if caption_extension.to_string() == "srt" || caption_extension.to_string() == "vtt"
                {
                    let caption_url = &caption.url;
                    let response = reqwest::get(caption_url).await?.text().await?;
                    let mut caption_file_path =
                        "output/".to_string() + &video_downloader.output_file_name;
                    if caption_languages.contains("orig") {
                        let original_language = caption_languages.replace("-orig", "");
                        caption_file_path = caption_file_path + "." + &original_language + ".";
                    } else {
                        caption_file_path = caption_file_path + ".en.";
                    }

                    caption_file_path = caption_file_path + caption_extension.to_string().as_str();

                    fs::write(caption_file_path, response).unwrap();
                }
            }
        }
    }

    Ok(video_downloader.video_path.clone().unwrap())
}

fn sanitize_filename(input: &str) -> String {
    let re = regex::Regex::new(r"[^ a-zA-Z0-9_\.-]").unwrap();
    re.replace_all(input, "").to_string()
}
