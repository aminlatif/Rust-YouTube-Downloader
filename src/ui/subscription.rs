use super::message::Message;

use iced::Subscription;

use iced::futures::stream;

fn some_worker(tx: tokio::sync::broadcast::Sender<Message>) -> impl stream::Stream<Item = Message> {
    let mut rx2 = tx.subscribe();
    async_stream::stream! {
        loop {
         let msg_result = rx2.recv().await;
            match msg_result {
                Ok(msg) => {yield msg},
                Err(_) => break
            }
        }
    }
}

pub fn subscription(tx: tokio::sync::broadcast::Sender<Message>) -> Subscription<Message> {
    let subscription: iced_futures::subscription::Subscription<Message> =
        Subscription::run_with_id("worker", some_worker(tx));
    subscription
}
