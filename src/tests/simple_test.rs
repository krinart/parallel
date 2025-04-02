use std::time::Duration;

#[tokio::test]
async fn test_simple() {
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_eq!(2 + 2, 4);
} 