use super::{message::Message as UIMessage, state::DownloaderUIState};
use iced::{
    widget::{
        button, column, combo_box, container, progress_bar, row, text, text_input, Column, Image,
        Row, Scrollable, Text,
    },
    Alignment, Element, Length,
};

pub fn view(downloader_ui_state: &DownloaderUIState) -> Element<UIMessage> {
    let main_column = Column::new()
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
                )))
                .push(match &downloader_ui_state.disabled {
                    true => button("Install Libraries"),
                    false => button("Install Libraries").on_press(UIMessage::InstallLibraries),
                })
                .push(match &downloader_ui_state.disabled {
                    true => button("Update Libraries"),
                    false => button("Update Libraries").on_press(UIMessage::UpdateLibraries),
                })
                .align_y(Alignment::Center),
        )
        .push(
            Row::new()
                .spacing(10.0)
                .push(match &downloader_ui_state.disabled {
                    true => {
                        text_input("Video URL", downloader_ui_state.video_url.as_str()).size(16)
                    }
                    false => text_input("Video URL", downloader_ui_state.video_url.as_str())
                        .size(16)
                        .on_input(|entered_text| UIMessage::UrlChanged(entered_text)),
                })
                .push(match &downloader_ui_state.disabled {
                    true => button("Get Info"),
                    false => button("Get Info").on_press(UIMessage::FetchInfo),
                }),
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
        .push(combo_box(
            &downloader_ui_state.format_selection_list_video,
            "Select Video format...",
            downloader_ui_state.selected_format_video.as_ref(),
            |format| UIMessage::SelectVideoFormat(format),
        ))
        .push(
            Row::new()
                .spacing(10.0)
                .push(combo_box(
                    &downloader_ui_state.format_selection_list_audio,
                    "Select Audio format...",
                    downloader_ui_state.selected_format_audio.as_ref(),
                    |format| UIMessage::SelectAudioFormat(format),
                ))
                .push(
                    if downloader_ui_state.show_download_button
                        && !downloader_ui_state.is_video_downloading
                        && !downloader_ui_state.is_video_downloaded
                    {
                        match &downloader_ui_state.disabled {
                            true => button("Download Video"),
                            false => button("Download Video").on_press(UIMessage::DownloadVideo),
                        }
                    } else if downloader_ui_state.is_video_downloading {
                        button("Downloading Video, Please Wait...")
                    } else if downloader_ui_state.is_video_downloaded {
                        button("Download Completed")
                    } else {
                        button("Download Video").height(0.0)
                    },
                ),
        )
        .push(
            Row::new()
                .spacing(10.0)
                .push(progress_bar(0.0..=100.0, downloader_ui_state.progress)),
        );
    let main_container = container(main_column)
        .width(Length::Fill)
        .height(Length::Fill);

    let status_bar: iced::widget::Row<'_, UIMessage, _, _> =
        row![text(downloader_ui_state.status_message.clone()).size(12)]
            .spacing(10)
            .align_y(Alignment::Center);
    let status_bar_container = container(status_bar).padding(8).width(Length::Fill);

    column![main_container, status_bar_container].into()
}
