use super::super::downloader::{
    change_video_url, download_video, get_video_info, get_video_thumbnail,
};
use super::{message::Message, state::DownloaderUIState};
use iced::Task;
use std::sync::Arc;

pub fn update(downloader_ui_state: &mut DownloaderUIState, message: Message) -> Task<Message> {
    match message {
        Message::UrlChanged(url) => {
            let url_copy = url.clone();
            downloader_ui_state.video_url = url;
            let video_downloader = Arc::clone(&downloader_ui_state.video_downloader); //.clone();
            Task::perform(
                async move {
                    let video_downloader_locked = video_downloader.write().await;
                    let _ = change_video_url(video_downloader_locked, url_copy);
                    let video_downloader_read = video_downloader.read().await;
                    video_downloader_read.video_info.clone()
                },
                |video_info| Message::UIUpdated,
            )
        }
        Message::FetchInfo => {
            downloader_ui_state.sender.as_ref().unwrap().send(super::message::Message::FetchInfo);
            Task::none()
            // let video_downloader = Arc::clone(&downloader_ui_state.video_downloader); //.clone();
            // Task::perform(
            //     async move {
            //         let video_downloader_locked = video_downloader.write().await;
            //         let _ = get_video_info(video_downloader_locked).await;
            //         let video_downloader_read = video_downloader.read().await;
            //         video_downloader_read.video_info.clone()
            //     },
            //     |video_info| Message::InfoFetched(video_info),
            // )
        }
        Message::InfoFetched(video_info) => {
            downloader_ui_state.video_id = video_info.id;
            downloader_ui_state.video_title = video_info.title;
            downloader_ui_state.video_channel = video_info.channel;
            downloader_ui_state.video_channel_id = video_info.channel_id;
            downloader_ui_state.video_description = video_info.description;
            // // Content::with_text(video.description.as_str());
            // // self.formats = video.formats;
            // downloader_ui.video_tags = downloader_ui.video_downloader.video_info.unwrap().tags;
            // downloader_ui.video_categories = downloader_ui.video_downloader.video_info.unwrap().categories;
            // println!("Info Fetched");
            // Task::perform(
            //     async move {
            //         tokio::spawn(async move || {
            //             downloader_ui.video_id = downloader_ui.video_downloader.read().await.video_info.unwrap().id;
            //         }).await.unwrap();
            //     }
            // )
            // self.video_info.perform(Action::Edit(Edit)); = format!("{:#?}", video).into();
            // downloader_ui.video_id = downloader_ui.video_downloader.read().video_info.unwrap().id;

            // // self.yt_dlp_version = video.yt_dlp_version;

            Task::done(Message::FetchThumbnail)
        }
        Message::FetchThumbnail => {
            let video_downloader = Arc::clone(&downloader_ui_state.video_downloader); //.clone();
            Task::perform(
                async move {
                    let video_downloader_locked = video_downloader.write().await;
                    let _ = get_video_thumbnail(video_downloader_locked).await;
                    let video_downloader_read = video_downloader.read().await;
                    video_downloader_read.thumbnail_path.clone()
                },
                |thumbnail_path| Message::ThumbnailFetched(thumbnail_path),
            )
        }
        Message::ThumbnailFetched(thumbnail_path) => {
            println!("Thumbnail Fetched");
            downloader_ui_state.thumbnail_path = thumbnail_path;
            downloader_ui_state.show_download_button = true;
            Task::none()
        }
        Message::DownloadVideo => {
            downloader_ui_state.is_video_downloading = true;
            let video_downloader = Arc::clone(&downloader_ui_state.video_downloader); //.clone();
            Task::perform(
                async move {
                    let video_downloader_locked = video_downloader.write().await;
                    let _ = download_video(video_downloader_locked).await;
                    let video_downloader_read = video_downloader.read().await;
                    video_downloader_read.video_path.clone()
                },
                |video_path| Message::VideoDownloaded(video_path),
            )
        }
        Message::VideoDownloaded(video_path) => {
            println!("Video Downloaded");
            downloader_ui_state.video_path = video_path;
            downloader_ui_state.is_video_downloading = false;
            downloader_ui_state.is_video_downloaded = true;

            Task::none()
        }

        Message::WindowResized(size) => {
            println!("Window Resized: {}", size.width);

            Task::none()
        }

        Message::Number(num) => {
            println!("UI Recieved number: {}", num);
            Task::none()
        }

        // Message::Increment => {
        //     self.num += 1;
        //     Task::none()
        // }
        // Message::Decrement => {
        //     self.num -= 1;
        //     Task::none()
        // }
        _ => Task::none(),
    }
}
