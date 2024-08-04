use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use sync::producer::Producer;
use tokio::sync::Mutex;
use tokio::sync::{broadcast, mpsc};
use tokio::time::Duration;

type Message = Vec<String>;

async fn async_producer_callback_stub(
    n: i32,
    callback_invocations: Arc<Mutex<i32>>,
    produced_items: Arc<Mutex<Vec<Message>>>,
) -> Message {
    let mut invocations = callback_invocations.lock().await;
    *invocations += 1;
    // Simulate async work
    // FIXME! the test driver should adjust the sleep time considering the sleep
    tokio::time::sleep(Duration::from_millis(100)).await;
    let items: Message = (0..n).map(|i| format!("item-{}", i)).collect();
    produced_items.lock().await.push(items.clone());
    println!("Produced: {:?}", &items);
    items
}

#[tokio::test]
async fn test_producer() {
    // Shared state for the test
    let callback_invocations = Arc::new(Mutex::new(0));
    let produced_items: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(Vec::new()));
    let items_per_invocation = 3;

    // Define the callback
    let callback_invocations_clone = callback_invocations.clone();
    let produced_items_clone = produced_items.clone();
    let callback = Arc::new(move || {
        let callback_invocations = callback_invocations_clone.clone();
        let produced_items = produced_items_clone.clone();
        Box::pin(async_producer_callback_stub(
            items_per_invocation,
            callback_invocations,
            produced_items,
        )) as Pin<Box<dyn Future<Output = Message> + Send>>
    });

    // Create a channel and a shutdown signal
    // note: the channel's capacity should be adjusted based on expected_nbr_of_calls (rate_limit * sleep_duration)
    let (tx, mut rx) = mpsc::channel(100);
    let (shutdown_tx, _) = broadcast::channel(1);

    // Create the producer
    let rate_limit = 5;
    let producer = Producer::new(rate_limit, callback, tx, shutdown_tx.clone());

    // Spawn the producer
    let producer_handle = tokio::spawn(async move {
        if let Err(e) = producer.run().await {
            eprintln!("Producer failed: {:?}", e);
        }
    });

    // Run for over 3 seconds
    let sleep_duration = 3;
    tokio::time::sleep(Duration::from_secs(sleep_duration)).await;

    // Trigger shutdown
    let _ = shutdown_tx.send(());

    // Stop the producer
    producer_handle.abort();

    // Check callback invocation
    let invocations = callback_invocations.lock().await;
    let expected_nbr_of_calls = rate_limit as i32 * sleep_duration as i32;
    assert_eq!(
        *invocations, expected_nbr_of_calls,
        "Callback should be called {} times",
        expected_nbr_of_calls
    );

    // Collect all produced items
    let mut collected_items = Vec::new();
    while let Some(item) = rx.recv().await {
        collected_items.push(item);
    }

    // Check produced items
    let produced_items = produced_items.lock().await;
    assert_eq!(
        collected_items.len(),
        produced_items.len(),
        "Produced items count mismatch"
    );
    assert!(
        collected_items.len() == expected_nbr_of_calls as usize,
        "Produced items should be {}, but got {:?}",
        expected_nbr_of_calls,
        &collected_items
    );
    assert_eq!(
        collected_items.as_slice(),
        produced_items.as_slice(),
        "Produced items mismatch"
    );
}
