use super::{message::Message, state::DownloaderUIState};
use iced::{
    widget::{button, text, text_input, Column, Image, Row, Scrollable, Text},
    Element,
};

pub fn view(downloader_ui_state: &DownloaderUIState) -> Element<Message> {
    Column::new()
        .padding(20.0)
        .spacing(10.0)
        .push(
            Row::new()
                .spacing(30.0)
                .push(text(format!(
                    "executables dir: ./{}",
                    downloader_ui_state.executables_dir
                )))
                .push(text(format!(
                    "output dir: ./{}",
                    downloader_ui_state.output_dir
                ))),
        )
        .push(
            Row::new()
                .spacing(10.0)
                .push(
                    text_input("Video URL", downloader_ui_state.video_url.as_str())
                        .size(16)
                        .on_input(|entered_text| Message::UrlChanged(entered_text)),
                )
                .push(button("Get Info").on_press(Message::FetchInfo)),
        )
        .push(
            Row::new()
                .spacing(10.0)
                .push(
                    Image::new(
                        downloader_ui_state
                            .thumbnail_path
                            .clone()
                            .unwrap_or("".into()),
                    )
                    .width(150.0),
                )
                .push(
                    Column::new()
                        .spacing(10.0)
                        .push(
                            Row::new()
                                .spacing(10.0)
                                .push(text(&downloader_ui_state.video_title).size(25))
                                .push(text(&downloader_ui_state.video_id).size(10)),
                        )
                        .push(
                            Row::new()
                                .spacing(10.0)
                                .push(text(&downloader_ui_state.video_channel).size(14))
                                .push(text(&downloader_ui_state.video_channel_id).size(14)),
                        ),
                ),
        )
        .push(
            Scrollable::new(Text::new(&downloader_ui_state.video_description).size(14))
                .height(150.0),
        )
        .push(
            if downloader_ui_state.show_download_button
                && !downloader_ui_state.is_video_downloading
                && !downloader_ui_state.is_video_downloaded
            {
                button("Download Video").on_press(Message::DownloadVideo)
            } else if downloader_ui_state.is_video_downloading {
                button("Downloading Video, Please Wait...")
            } else if downloader_ui_state.is_video_downloaded {
                button("Download Completed")
            } else {
                button("Download Video").height(0.0)
            },
        )
        .into()
}
