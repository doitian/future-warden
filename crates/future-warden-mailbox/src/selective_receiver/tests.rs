use super::*;

use crate::tokio_impl::mailbox;

#[tokio::test]
async fn test_selective_receiver_immediate_match() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    tx.send(0).await.unwrap();
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();

    let result = selective_rx.recv_selectively(|&x| x > -1).await.unwrap();
    assert_eq!(result, Some(0));
}

#[tokio::test]
async fn test_selective_receiver_buffering() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    tx.send(0).await.unwrap();
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    tx.send(3).await.unwrap();
    tx.send(4).await.unwrap();

    let result = selective_rx.recv_selectively(|&x| x > 2).await.unwrap();
    assert_eq!(result, Some(3));

    assert_eq!(selective_rx.recv().await.unwrap(), 0);
    assert_eq!(selective_rx.recv().await.unwrap(), 1);
    assert_eq!(selective_rx.recv().await.unwrap(), 2);
    assert_eq!(selective_rx.recv().await.unwrap(), 4);
}

#[tokio::test]
async fn test_selective_receiver_no_match_until_capacity() {
    let (tx, rx) = mailbox(4);
    let mut selective_rx = SelectiveReceiver::new(rx);

    let task = tokio::spawn(async move {
        tx.send(0).await.unwrap();
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
        tx.send(4).await.unwrap();
    });

    let result = selective_rx.recv_selectively(|&x| x > 9).await.unwrap();
    task.await.unwrap();

    assert_eq!(result, None);

    assert_eq!(selective_rx.recv().await.unwrap(), 0);
    assert_eq!(selective_rx.recv().await.unwrap(), 1);
    assert_eq!(selective_rx.recv().await.unwrap(), 2);
    assert_eq!(selective_rx.recv().await.unwrap(), 3);
    assert_eq!(selective_rx.recv().await.unwrap(), 4);
}

#[tokio::test]
async fn test_selective_receiver_multiple_selects() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    for i in 1..10 {
        tx.send(i).await.unwrap();
    }

    // Select even numbers
    assert_eq!(
        selective_rx
            .recv_selectively(|&x| x % 2 == 0)
            .await
            .unwrap(),
        Some(2)
    );
    assert_eq!(
        selective_rx
            .recv_selectively(|&x| x % 2 == 0)
            .await
            .unwrap(),
        Some(4)
    );

    // Select odd numbers from buffer
    assert_eq!(
        selective_rx
            .recv_selectively(|&x| x % 2 == 1)
            .await
            .unwrap(),
        Some(1)
    );
    assert_eq!(
        selective_rx
            .recv_selectively(|&x| x % 2 == 1)
            .await
            .unwrap(),
        Some(3)
    );

    // Continue selecting even
    assert_eq!(
        selective_rx
            .recv_selectively(|&x| x % 2 == 0)
            .await
            .unwrap(),
        Some(6)
    );
}

#[tokio::test]
async fn test_selective_receiver_closed_mailbox() {
    let (tx, rx) = mailbox::<i32>(10);
    let mut selective_rx = SelectiveReceiver::new(rx);

    drop(tx);

    let result = selective_rx.recv_selectively(|&x| x > -1).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_selective_receiver_select_from_buffer() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    tx.send(0).await.unwrap();
    tx.send(1).await.unwrap();
    tx.send(4).await.unwrap();
    tx.send(2).await.unwrap();

    assert_eq!(
        selective_rx.recv_selectively(|&x| x > 3).await.unwrap(),
        Some(4)
    );

    assert_eq!(
        selective_rx.recv_selectively(|&x| x == 2).await.unwrap(),
        Some(2)
    );

    assert_eq!(selective_rx.recv().await.unwrap(), 0);
    assert_eq!(selective_rx.recv().await.unwrap(), 1);
}

#[tokio::test]
async fn test_selective_receiver_tombstone_handling() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    // Send messages
    tx.send(9).await.unwrap();
    tx.send(19).await.unwrap();
    tx.send(29).await.unwrap();
    tx.send(39).await.unwrap();

    // Select 29, which should buffer 9, 19
    assert_eq!(
        selective_rx.recv_selectively(|&x| x == 29).await.unwrap(),
        Some(29)
    );

    // Select 9 from buffer
    assert_eq!(
        selective_rx.recv_selectively(|&x| x == 9).await.unwrap(),
        Some(9)
    );

    // Now recv should skip the tombstone and get 19
    assert_eq!(selective_rx.recv().await.unwrap(), 19);
    assert_eq!(selective_rx.recv().await.unwrap(), 39);
}

#[tokio::test]
async fn test_selective_receiver_preserves_receiver_trait() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    // Test that SelectiveReceiver implements Receiver trait correctly
    assert_eq!(selective_rx.max_capacity(), 9);

    tx.send(0).await.unwrap();
    tx.send(1).await.unwrap();

    // Use Receiver trait methods
    assert_eq!(Receiver::recv(&mut selective_rx).await.unwrap(), 0);
    assert_eq!(Receiver::recv(&mut selective_rx).await.unwrap(), 1);
}

#[tokio::test]
async fn test_selective_receiver_complex_predicate() {
    let (tx, rx) = mailbox(9);
    let mut selective_rx = SelectiveReceiver::new(rx);

    // Send various messages
    tx.send(16).await.unwrap();
    tx.send(6).await.unwrap();
    tx.send(21).await.unwrap();
    tx.send(8).await.unwrap();
    tx.send(29).await.unwrap();

    // Select first number divisible by 4 and greater than 10
    let result = selective_rx
        .recv_selectively(|&x| x % 4 == 0 && x > 10)
        .await
        .unwrap();
    assert_eq!(result, Some(16));

    // Select first number less than 9
    let result = selective_rx.recv_selectively(|&x| x < 9).await.unwrap();
    assert_eq!(result, Some(6));

    // Remaining messages
    assert_eq!(selective_rx.recv().await.unwrap(), 21);
    assert_eq!(selective_rx.recv().await.unwrap(), 8);
    assert_eq!(selective_rx.recv().await.unwrap(), 29);
}
