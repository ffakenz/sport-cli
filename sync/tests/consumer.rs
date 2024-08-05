use std::sync::Arc;
use sync::consumer::Consumer;
use tokio::sync::Mutex;
use tokio::sync::{broadcast, mpsc};
use tokio::time::Duration;

type Message = String;

async fn async_consumer_callback_stub(
    message: Message,
    callback_invocations: Arc<Mutex<i32>>,
    processed_items: Arc<Mutex<Vec<Message>>>,
) {
    let mut invocations = callback_invocations.lock().await;
    *invocations += 1;
    // Simulate async work
    // FIXME! the test driver should adjust the sleep time considering the sleep
    tokio::time::sleep(Duration::from_millis(100)).await;
    processed_items.lock().await.push(message.clone());
    println!("Consumed: {:?}", message);
}

#[tokio::test]
async fn test_consumer() {
    // Shared state for the test
    let callback_invocations = Arc::new(Mutex::new(0));
    let processed_items: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(Vec::new()));

    // Create a channel and a shutdown signal
    // note: the channel's capacity should be adjusted based on messages_to_send
    let (tx, rx) = mpsc::channel(100);
    let (shutdown_tx, _) = broadcast::channel(1);

    // Create the consumer
    let rate_limit = 5;
    // Define the callback
    let callback_invocations_clone = Arc::clone(&callback_invocations);
    let processed_items_clone = Arc::clone(&processed_items);

    // Spawn the consumer
    let consumer_handle = Consumer::spawn(
        rate_limit,
        rx,
        shutdown_tx.clone(),
        move |message: Message| {
            let callback_invocations = Arc::clone(&callback_invocations_clone);
            let processed_items = Arc::clone(&processed_items_clone);
            async move {
                async_consumer_callback_stub(message, callback_invocations, processed_items).await
            }
        },
    );

    // Send some messages
    let nbr_of_msgs = 50;
    let messages_to_send: Vec<String> = (0..nbr_of_msgs).map(|i| format!("item-{}", i)).collect();
    for msg in messages_to_send.clone() {
        tx.send(msg).await.unwrap();
    }

    // Run for over 3 seconds
    let sleep_duration = 3;
    tokio::time::sleep(Duration::from_secs(sleep_duration)).await;

    // Trigger shutdown
    let _ = shutdown_tx.send(());

    // Stop the consumer
    consumer_handle.abort();

    // Check callback invocation
    let invocations = callback_invocations.lock().await;
    let expected_nbr_of_calls = rate_limit as i32 * sleep_duration as i32;
    assert_eq!(
        *invocations, expected_nbr_of_calls,
        "Callback should be called {} times",
        expected_nbr_of_calls
    );

    // Check processed items
    let processed_items = processed_items.lock().await;
    assert_eq!(
        *invocations as usize,
        processed_items.len(),
        "Processed items count mismatch"
    );
    let expected_messages_processed: Vec<&Message> = messages_to_send
        .iter()
        .take(processed_items.len())
        .collect();

    let expected_processed_messages: Vec<&Message> = processed_items.iter().collect();
    assert_eq!(
        expected_messages_processed.as_slice(),
        expected_processed_messages.as_slice(),
        "Processed items mismatch"
    );
}
