use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::Output;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockWriteGuard};

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;
use tracing_subscriber::fmt::format;
use yt_dlp::fetcher::thumbnail;
use yt_dlp::model::format::VideoResolution;
use yt_dlp::model::ExtractorInfo;
use yt_dlp::model::Version;
use yt_dlp::model::Video;
use yt_dlp::{
    fetcher::deps::{Libraries, LibraryInstaller},
    Youtube,
};

#[derive(Debug)]
pub struct DownloadedVideoInfo {
    thumbnail_path: Option<PathBuf>,
    video_path: Option<PathBuf>,
}

pub async fn download(
    executables_dir: &PathBuf,
    output_dir: &PathBuf,
    video_url: &str,
) -> Result<DownloadedVideoInfo, yt_dlp::error::Error> {
    let youtube = executables_dir.join("yt-dlp");
    let ffmpeg = executables_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir)?;

    let url = String::from(video_url);

    let mut output_file_name = url.split('/').last().unwrap().to_string();

    let video_infos: Option<Video> = match fetcher.fetch_video_infos(url.clone()).await {
        Ok(video_infos) => {
            println!("Video infos recieved.");
            output_file_name = video_infos.title.clone();
            Some(video_infos)
        }
        Err(e) => {
            error!("Error fetching video infos: {}", e);
            None
        }
    };

    for caption_group in video_infos.unwrap().automatic_captions {
        let caption_languages = caption_group.0;
        if caption_languages.contains("en") || caption_languages.contains("orig") {
            for caption in caption_group.1 {
                let caption_extension = caption.extension;
                if caption_extension.to_string() == "srt" || caption_extension.to_string() == "vtt"
                {
                    let caption_url = caption.url;
                    let response = reqwest::get(caption_url).await?.text().await?;
                    let mut caption_file_path = "output/".to_string() + &output_file_name;
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

    let thumbnail_path: Option<PathBuf> = match fetcher
        .download_thumbnail_from_url(url.clone(), output_file_name.clone() + ".jpg")
        .await
    {
        Ok(path) => {
            println!("Thumbnail downloaded to: {}", path.display());
            Some(path)
        }
        Err(e) => {
            error!("Error downloading thumbnail: {}", e);
            None
        }
    };

    let video_path: Option<PathBuf> = match fetcher
        .download_video_from_url(url.clone(), output_file_name.clone() + ".mp4")
        .await
    {
        Ok(path) => {
            println!("Video downloaded to: {}", path.display());
            Some(path)
        }
        Err(e) => {
            error!("Error downloading video: {}", e);
            None
        }
    };

    for entry in fs::read_dir("output")? {
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

    Ok(DownloadedVideoInfo {
        thumbnail_path,
        video_path,
    })
}

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

pub fn change_video_url(
    mut video_downloader: RwLockWriteGuard<'_, VideoDownloader>,
    video_url: String,
) {
    video_downloader.video_url = video_url;
}

pub async fn get_video_info(
    mut video_downloader: RwLockWriteGuard<'_, VideoDownloader>,
) -> anyhow::Result<()> {
    match video_downloader
        .fetcher
        .fetch_video_infos(video_downloader.video_url.clone())
        .await
    {
        Ok(video_info) => {
            println!("Video infos recieved.");
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
            Ok(())
        }
        Err(e) => {
            error!("Error fetching video infos: {}", e);
            Err(e.into())
        }
    }
}

pub async fn get_video_thumbnail(
    mut video_downloader: RwLockWriteGuard<'_, VideoDownloader>,
) -> anyhow::Result<()> {
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
            Ok(())
        }
        Err(e) => {
            error!("Error downloading thumbnail: {}", e);
            Err(e.into())
        }
    }
}

pub async fn download_video(
    mut video_downloader: RwLockWriteGuard<'_, VideoDownloader>,
) -> anyhow::Result<()> {
    // video_downloader.video_path = match video_downloader.fetcher
    //     .download_video_from_url(video_downloader.video_url.clone(), video_downloader.output_file_name.clone() + ".mp4")
    //     .await
    // {
    //     Ok(path) => {
    //         println!("Video downloaded to: {}", path.display());
    //         Some(path)
    //     }
    //     Err(e) => {
    //         error!("Error downloading video: {}", e);
    //         None
    //     }
    // };
    // tokio::task::spawn(async move || {
    //     let download_id = video_downloader
    //         .fetcher
    //         .download_video_with_progress(
    //             &video_downloader.video_info,
    //             video_downloader.output_file_name.clone() + ".mp4",
    //             |downloaded, total| {
    //                 println!("Progress: {}/{}", downloaded, total);
    //                 // let percentage = if total > 0 {
    //                 //     (downloaded as f64 / total as f64 * 100.0) as u64
    //                 // } else {
    //                 //     0
    //                 // };
    //                 // println!("Progress: {}/{} bytes ({}%)", downloaded, total, percentage);
    //             },
    //         )
    //         .await
    //         .unwrap();
    //     println!("Download ID: {}", download_id);

    //     video_downloader
    //         .fetcher
    //         .wait_for_download(download_id)
    //         .await;
    // })
    // .await
    // .unwrap();

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

    Ok(())
}

fn sanitize_filename(input: &str) -> String {
    // Keep only alphanumeric, dash, underscore, and dot
    let re = regex::Regex::new(r"[^ a-zA-Z0-9_\.-]").unwrap();
    re.replace_all(input, "").to_string()
}
