mod downloader;
mod installer;
// mod ui;
mod ui;

use iced::advanced::graphics::text::cosmic_text::ttf_parser::math;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
use ui::state::DownloaderUIState;

use crate::ui::subscription::subscription;

use tokio::sync::Mutex;

fn main() -> iced::Result {
    let (ui_to_worker_tx, mut worker_from_ui_rx_1) =
        tokio::sync::broadcast::channel::<ui::message::Message>(16);
    let (worker_to_ui_tx, mut ui_from_worker_rx_1) =
        tokio::sync::broadcast::channel::<ui::message::Message>(16);

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    let result_msg = worker_from_ui_rx_1.recv().await;
                    match result_msg {
                        Ok(msg) => {
                            println!("Worker received message: {:?}", msg);
                        }
                        Err(e) => {
                            println!("Worder received error: {}", e);
                        }
                    }
                }
            })
    });

    let worker_to_ui_tx_2 = worker_to_ui_tx.clone();

    std::thread::spawn(move || {
        let mut n = 0;
        loop {
            n += 1;
            println!("sending from worker thread {}", n);
            let _ = worker_to_ui_tx.send(ui::message::Message::Number(n));
            // tx.send(ui::message::Message::Number(n));
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    });

    let mut iced_application =
        iced::application("YouTube Downloader", ui::update::update, ui::view::view)
            // .subscription(ui::subscription::subscription)
            // .subscription(move |_| subscription(Arc::clone(&ui_from_worker_receiver)))
            .subscription(move |_| subscription(worker_to_ui_tx_2.clone()))
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

    // let downloader_ui = ui::DownloaderUI::new();
    // iced::application(DownloaderUIState:default, DownloaderUI::update, DownloaderUI::view);
    // || rx, ui::update::update, ui::view::view);
    // let _ = iced::run("YouTube Downloader", ui::update::update, ui::view::view);
    // tracing_subscriber::fmt::init();

    // match ui::show_window() {
    //     Ok(_) => println!("Window closed"),
    //     Err(e) => println!("Error showing window: {}", e),
    // };

    // let executables_dir = PathBuf::from("libs");
    // let output_dir = PathBuf::from("output");
    // let video_url = String::from("https://www.youtube.com/watch?v=fy1B67GxK-0");
    // // let video_url = String::from("https://www.youtube.com/watch?v=ly6YKz9UfQ4");
    // // let video_url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    // // let video_url = String::from("https://www.youtube.com/watch?v=ly6YKz9UfQ4");
    // // let video_url = String::from("https://www.youtube.com/watch?v=xEAQm8Op3Hw");

    // match download::download(&executables_dir, &output_dir, &video_url).await {
    //     Ok(video_path) => println!("Video downloaded to: {:#?}", video_path),
    //     Err(e) => println!("Error downloading video: {}", e),
    // }

    Ok(())
}
