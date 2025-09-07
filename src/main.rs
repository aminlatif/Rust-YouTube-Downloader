mod downloader;
mod installer;
mod ui;

use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::{debug, warn};
use tracing_subscriber::filter::EnvFilter;
use ui::{
    message::Message as UIMessage, state::DownloaderUIState,
    subscription::subscription as ui_subscription,
};

use downloader::{change_video_url, download_video, get_video_info, get_video_thumbnail};

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_env_filter(EnvFilter::new("iced=off,youtube_downloader=debug"))
        .init();

    let (ui_to_worker_tx, mut worker_from_ui_rx_1) =
        tokio::sync::broadcast::channel::<ui::message::Message>(16);
    let (worker_to_ui_tx, mut ui_from_worker_rx_1) =
        tokio::sync::broadcast::channel::<ui::message::Message>(16);

    let worker_to_ui_tx_2 = worker_to_ui_tx.clone();
    let mut worker_from_ui_rx_2 = ui_to_worker_tx.subscribe();

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let mut video_downloader = downloader::VideoDownloader::default();
                let worker_to_ui_tx_3 = worker_to_ui_tx.clone();
                loop {
                    let result_msg = worker_from_ui_rx_2.recv().await;
                    match result_msg {
                        Ok(UIMessage::InstallLibraries) => {
                            debug!("Worker thread received InstallLibraries message");
                            let _ =
                                match installer::install(&video_downloader.executables_dir).await {
                                    Ok((_ffmpeg_path, _yt_dlp_path)) => {
                                        worker_to_ui_tx_3.send(UIMessage::LibrariesInstalled)
                                    }
                                    Err(e) => worker_to_ui_tx_3.send(UIMessage::StatusMessage(
                                        format!("Failed to install libraries: {}", e),
                                    )),
                                };
                        }
                        Ok(UIMessage::UpdateLibraries) => {
                            debug!("Worker thread received UpdateLibraries message");
                            let _ = match installer::update(&video_downloader.executables_dir).await
                            {
                                Ok(_yt_dlp_path) => {
                                    worker_to_ui_tx_3.send(UIMessage::LibrariesUpdated)
                                }
                                Err(e) => worker_to_ui_tx_3.send(UIMessage::StatusMessage(
                                    format!("Failed to update libraries: {}", e),
                                )),
                            };
                        }
                        Ok(UIMessage::UrlChanged(url)) => {
                            debug!("Worker thread received UrlChanged message: {}", url);
                            change_video_url(&mut video_downloader, url);
                        }
                        Ok(UIMessage::FetchInfo) => {
                            debug!("Worker thread received FetchInfo message.");
                            let _ = match get_video_info(&mut video_downloader).await {
                                Ok(video_info) => {
                                    worker_to_ui_tx_3.send(UIMessage::InfoFetched(video_info))
                                }
                                Err(e) => {
                                    worker_to_ui_tx_3.send(UIMessage::StatusMessage(
                                        format!("Failed to fetch video info: {}", e),
                                    ))
                                }
                            };
                        }
                        Ok(UIMessage::FetchThumbnail) => {
                            debug!("Worker thread received FetchThumbnail message.");
                            let _ = match get_video_thumbnail(&mut video_downloader).await {
                                Ok(path) => {
                                    worker_to_ui_tx_3.send(UIMessage::ThumbnailFetched(Some(path)))
                                }
                                Err(e) => {
                                    worker_to_ui_tx_3.send(UIMessage::StatusMessage(
                                        format!("Failed to fetch video thumbnail: {}", e),
                                    ))
                                }
                            };
                        }
                        Ok(UIMessage::DownloadVideo) => {
                            debug!("Worker thread received DownloadVideo message.");
                            let worker_to_ui_tx_4 = worker_to_ui_tx.clone();
                            let _ = match download_video(&mut video_downloader, &worker_to_ui_tx_4).await {
                                Ok(path) => {
                                    worker_to_ui_tx_3.send(UIMessage::VideoDownloaded(Some(path)))
                                }
                                Err(e) => {
                                    worker_to_ui_tx_3.send(UIMessage::StatusMessage(
                                        format!("Failed to download video: {}", e),
                                    ))
                                }
                            };
                        }
                        Ok(UIMessage::Number(number)) => {
                            debug!("Worker thread received number: {}", number);
                        }
                        Ok(msg) => {
                            warn!("Worker thread received unknown message: {:?}", msg);
                        }
                        Err(e) => {
                            warn!("Worker thread received error: {}", e);
                        }
                    };
                }
            })
    });

    // std::thread::spawn(move || {
    //     tokio::runtime::Builder::new_current_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on(async move {
    //             loop {
    //                 let result_msg = worker_from_ui_rx_1.recv().await;
    //                 match result_msg {
    //                     Ok(msg) => {
    //                         println!("Worker received message: {:?}", msg);
    //                     }
    //                     Err(e) => {
    //                         println!("Worder received error: {}", e);
    //                     }
    //                 }
    //             }
    //         })
    // });

    // std::thread::spawn(move || {
    //     let mut n = 0;
    //     loop {
    //         n += 1;
    //         println!("sending from worker thread {}", n);
    //         let _ = worker_to_ui_tx.send(ui::message::Message::Number(n));
    //         // tx.send(ui::message::Message::Number(n));
    //         std::thread::sleep(std::time::Duration::from_secs(10));
    //     }
    // });

    let mut iced_application =
        iced::application("YouTube Downloader", ui::update::update, ui::view::view)
            // .subscription(ui::subscription::subscription)
            // .subscription(move |_| subscription(Arc::clone(&ui_from_worker_receiver)))
            .subscription(move |_| ui_subscription(worker_to_ui_tx_2.clone()))
            .theme(|_| iced::Theme::Dark)
            .centered();
    let _ = iced_application.run_with(|| {
        (
            DownloaderUIState {
                sender: Some(ui_to_worker_tx),
                reciever: Some(ui_from_worker_rx_1),
                ..Default::default()
            },
            iced::Task::none(),
        )
    });

    Ok(())
}
