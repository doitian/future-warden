use super::*;

use crate::{Receiver, Sender};

#[tokio::test]
async fn test_mailbox() {
    let (tx, mut rx) = mailbox(50);
    assert_eq!(Receiver::len(&rx), 0);
    assert_eq!(Receiver::max_capacity(&rx), 50);

    assert!(Sender::send(&tx, 1).await.is_ok());
    assert_eq!(Receiver::len(&rx), 1);
    assert_eq!(Receiver::max_capacity(&rx), 50);

    assert_eq!(Receiver::recv(&mut rx).await, Ok(1));
    assert_eq!(Receiver::len(&rx), 0);
    assert_eq!(Receiver::max_capacity(&rx), 50);
}
