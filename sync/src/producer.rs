use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::time::Duration;

pub type ProducerCallback<T> =
    dyn Fn() -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync + 'static;

pub struct Producer<T: 'static> {
    producer_rate_limit: usize,
    producer_callback: Arc<ProducerCallback<T>>,
    tx: mpsc::Sender<T>,
    shutdown_tx: broadcast::Sender<()>,
}

impl<T> Producer<T> {
    pub fn new(
        producer_rate_limit: usize,
        producer_callback: Arc<ProducerCallback<T>>,
        tx: mpsc::Sender<T>,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Self {
        Self {
            producer_rate_limit,
            producer_callback,
            tx,
            shutdown_tx,
        }
    }
}

impl<T> Producer<T> {
    pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Rate limit enforcement
        let interval = Duration::from_secs(1) / self.producer_rate_limit as u32;
        let mut interval_timer = tokio::time::interval(interval);

        let mut shutdown_rx = self.shutdown_tx.subscribe();

        loop {
            tokio::select! {
                _ = interval_timer.tick() => {
                    // Execute the callback
                    let callback = self.producer_callback.clone();
                    let message = callback().await;

                    // Send message to the channel
                    if self.tx.send(message).await.is_err() {
                        // The receiver dropped
                        return Ok(());
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
