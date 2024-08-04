use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::time::Duration;

pub type ConsumerCallback<T> =
    dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static;

pub struct Consumer<T: 'static> {
    consumer_rate_limit: usize,
    consumer_callback: Arc<ConsumerCallback<T>>,
    rx: mpsc::Receiver<T>,
    shutdown_tx: broadcast::Sender<()>,
}

impl<T> Consumer<T> {
    pub fn new(
        consumer_rate_limit: usize,
        consumer_callback: Arc<ConsumerCallback<T>>,
        rx: mpsc::Receiver<T>,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Self {
        Self {
            consumer_rate_limit,
            consumer_callback,
            rx,
            shutdown_tx,
        }
    }
}

impl<T> Consumer<T> {
    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Rate limit enforcement
        let interval = Duration::from_secs(1) / self.consumer_rate_limit as u32;
        let mut interval_timer = tokio::time::interval(interval);

        let mut shutdown_rx = self.shutdown_tx.subscribe();

        loop {
            tokio::select! {
                _ = interval_timer.tick() => {
                    // Receive message from the channel
                    if let Some(data) =
                        // REVIEW! release await at interval duration
                        self.rx.recv().await {
                            // Execute the callback
                            let consumer_callback = self.consumer_callback.clone();
                            consumer_callback(data).await;
                        }
                }
                _ = shutdown_rx.recv() => {
                    // Shutdown signal received
                    break;
                }
            }
        }

        Ok(())
    }
}
