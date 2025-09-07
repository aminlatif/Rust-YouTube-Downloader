use std::fs;
use std::io;

use anyhow::Result;
use std::path::PathBuf;
use yt_dlp::{
    fetcher::deps::{Libraries, LibraryInstaller},
    Youtube,
};

pub async fn install(executables_dir: &PathBuf) {
    install_ffmpeg(&executables_dir)
        .await
        .expect("Failed to install ffmpeg");

    install_yt_dlp(&executables_dir)
        .await
        .expect("Failed to install yt-dlp");
}

pub async fn update(executables_dir: &PathBuf) {
    update_yt_dlp(&executables_dir)
        .await
        .expect("Failed to update yt-dlp");
}

pub async fn install_ffmpeg(executables_dir: &PathBuf) -> Result<PathBuf> {
    let ffmpeg_intaller = LibraryInstaller::new(executables_dir.clone());
    let ffmpeg = ffmpeg_intaller.install_ffmpeg(Some("ff".to_string())).await;
    match ffmpeg {
        Ok(_) => {
            println!("FFmpeg installed");
            return Ok(ffmpeg.unwrap());
        }
        Err(e) => {
            let source_directory = "libs\\ffmpeg-release\\ffmpeg-8.0-essentials_build\\bin";
            let destination_directory = "libs";

            println!("Moving to default ffmpeg path: from {source_directory} to {destination_directory}:");
            match fs::create_dir(destination_directory) {
                Ok(_) => println!("{destination_directory} directory created"),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                    println!("{destination_directory} directory already exists")
                }
                Err(e) => {
                    eprintln!("Failed to create directory: {e}");
                    return Err(e.into());
                }
            }

            for entry in fs::read_dir(source_directory)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if !file_type.is_dir() {
                    let destination_path  = PathBuf::from(destination_directory);
                    fs::copy(entry.path(), destination_path.join(entry.file_name()))?;
                }
            }

            match fs::remove_dir_all("libs\\ffmpeg-release") {
                Ok(_) => println!("libs\\ffmpeg-release directory removed"),
                Err(e) => eprintln!("Failed to remove directory libs\\ffmpeg-release: {e}"),
            }

            return Ok(PathBuf::from(
                "libs\\ffmpeg",
            ));
        }
    }
}

pub async fn install_yt_dlp(executables_dir: &PathBuf) -> Result<PathBuf> {
    let intaller = LibraryInstaller::new(executables_dir.clone());
    let yt_dlp = intaller.install_youtube(None).await;
    match yt_dlp {
        Ok(_) => {
            println!("yt-dlp installed");
            return Ok(yt_dlp.unwrap());
        }
        Err(e) => {
            panic!("yt-dlp installation failed: {}", e);
        }
    }
}

pub async fn update_yt_dlp(executables_dir: &PathBuf) -> Result<()> {
    let youtube = executables_dir.join("yt-dlp");
    let ffmpeg = executables_dir.join("ffmpeg");
    let output_dir = PathBuf::from("output");

    let libraries = Libraries::new(youtube, ffmpeg);

    let fetcher = Youtube::new(libraries, output_dir)?;

    fetcher.update_downloader().await?;

    Ok(())
}

