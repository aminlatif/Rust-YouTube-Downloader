use std::sync::Arc;

use super::{message::Message, state::DownloaderUIState};
use iced::futures::lock::MutexGuard;
use iced::window;

use iced::{Size, Subscription};

use iced::futures::{stream, StreamExt};

use std::sync::Mutex;

fn some_worker(
    // mut rx: tokio::sync::mpsc::UnboundedReceiver<Message>,
    // mut rx: tokio::sync::mpsc::UnboundedReceiver<Message>, // tokio::sync::mpsc::UnboundedReceiver<Message>,
    // mut rx: std::sync::MutexGuard<'a, tokio::sync::mpsc::UnboundedReceiver<Message>>
    // mut rx: Arc<Mutex<tokio::sync::mpsc::UnboundedReceiver<Message>>>,
    tx: tokio::sync::broadcast::Sender<Message>,
) -> impl stream::Stream<Item = Message> {
    let mut rx2 = tx.subscribe();
    //     // Stream::Item(Message::Increment)
    //     // let rx = *rx.clone();
    //     // let mut locked_rx = rx.lock().unwrap();
    //     // stream::unfold(init, f)
    async_stream::stream! {
        loop {
         let msg_result = rx2.recv().await;
            match msg_result {
                Ok(msg) => {yield msg},
                Err(_) => break
            }
            // yield msg;
        }
    }
    //     stream::unfold(rx, async move |mut rx| {
    //         let mut locked_rx = rx.lock().unwrap();
    //         match locked_rx.recv().await {
    //             Some(msg) => {
    //                 drop(locked_rx);
    //                 Some((msg, rx))
    //             },
    //             None => None,
    //         }
    //     })
    // stream::channel(100, async |mut output| {
    //     // Create channel
    //     let (sender, mut receiver) = mpsc::channel(100);

    //     // Send the sender back to the application
    //     output.send(Event::Ready(sender)).await;

    //     loop {
    //         use iced_futures::futures::StreamExt;

    //         // Read next input sent from `Application`
    //         let input = receiver.select_next_some().await;

    //         match input {
    //             Input::DoSomeWork => {
    //                 // Do some async work...

    //                 // Finally, we can optionally produce a message to tell the
    //                 // `Application` the work is done
    //                 output.send(Event::WorkFinished).await;
    //             }
    //         }
    //     }
    // })
}

// pub fn subscription(downloader_ui_state: &DownloaderUIState) -> Subscription<Message> {
pub fn subscription(
    // mut rx: tokio::sync::mpsc::UnboundedReceiver<Message>,
    // mut rx: Arc<Mutex<tokio::sync::mpsc::UnboundedReceiver<Message>>>,
    // downloader_ui_state: &DownloaderUIState,
    tx: tokio::sync::broadcast::Sender<Message>,
) -> Subscription<Message> {
    // let rx2 = rx.clone();

    let subscription: iced_futures::subscription::Subscription<Message> =
        Subscription::run_with_id("worker", some_worker(tx));
    // Subscription::run({
    //     let rx2 = tx.subscribe();
    //     // let locked_rx = rx2.lock().unwrap();
    //     // stream::empty().boxed()
    //     //
    //     stream::unfold(rx2, move |rx2| async move {
    //         match rx2.recv().await {
    //             Ok(msg) => Some((msg, rx2)),
    //             Err(_) => None,
    //         }
    //         // let value = locked_rx.lock().unwrap();
    //         // Some((Message::Increment, rx2))
    //     })
    //     .boxed()
    // });

    subscription
    // let mut rx = downloader_ui_state.reciever.clone(); // downloader_ui_state.reciever;
    // let locked_rx = rx.lock().unwrap();
    // async_stream::stream! {
    //     while let Some(msg) = rx.recv().await {
    //         yield msg;
    //     }
    // }
    // Subscription::run_with_id("worker", some_worker(rx))
    // Subscription::run_with_id("worker", move || {
    //     async_stream::stream! {
    //         while let Some(msg) = rx.recv().await {
    //             yield msg;
    //         }
    //     }
    // stream::unfold(locked_rx, |mut rx| async move {
    //     match rx.recv().await {
    //         Some(msg) => Some((msg, rx)),
    //         None => None,
    //     }
    // })
    // })
    // Subscription::run(move || {
    //     stream::unfold((), move |_| async move {
    //         let value = *rx.lock().unwrap();
    //         iced::futures::future::ready(Some((value, ())))
    //     })
    // })
    // tokio_stream::Stream::new();
    // let mut rx = downloader_ui_state.reciever.clone(); // downloader_ui_state.reciever;
    // let locked_rx = rx.to_owned();
    // Subscription::run_with_id("worker", some_worker(locked_rx))

    // iced::Subscription::run_with_id("worker", move || async_stream::stream! {
    //     while let Some(value) = rx.recv().await {
    //         yield Message::Increment; // yield value; // feed each value into Iced's update()
    //     }
    // })
    // iced::Subscription::run_with_id("worker", move || {
    //     // let mut worker_to_ui = downloader_ui_state.reciever.clone().unwrap();
    //     stream::unfold(worker_to_ui, |mut worker_to_ui| async move {

    //         // worker_to_ui.recv().await.map(|msg| (msg, worker_to_ui))
    //     })

    //     // stream::unfold()
    //     // println!("running worker");
    //     // stream::empty()
    // })
    // iced::Subscription:: ::unfold(
    //     "worker",
    //     state.receiver.clone(),
    //     |mut rx| async move {
    //         match rx.recv().await {
    //             Some(msg) => (Some(msg), rx),
    //             None => (None, rx),
    //         }
    //     },
    // );

    // window::resize_events().map(|(_id, size)| {
    //     // downloader_ui_state.sender.unwrap().send(50);
    //     Message::WindowResized(size)
    // })
}
