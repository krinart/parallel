use crate::{get_posts, get_followers};
use parallel_macro::parallel;
use std::time::Duration;

async fn quick_task() -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    "quick".to_string()
}

async fn slow_task() -> String {
    tokio::time::sleep(Duration::from_millis(200)).await;
    "slow".to_string()
}

async fn error_task() -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    Err("error occurred".to_string())
}

#[tokio::test]
async fn test_parallel_basic() {
    let (result1, result2) = parallel! {
        quick_task(),
        slow_task()
    };
    
    assert_eq!(result1, "quick");
    assert_eq!(result2, "slow");
}

#[tokio::test]
async fn test_parallel_three_tasks() {
    let (result1, result2, result3) = parallel! {
        quick_task(),
        slow_task(),
        quick_task()
    };
    
    assert_eq!(result1, "quick");
    assert_eq!(result2, "slow");
    assert_eq!(result3, "quick");
}

#[tokio::test]
async fn test_parallel_with_different_types() {
    async fn number_task() -> i32 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        42
    }

    let (string_result, number_result) = parallel! {
        quick_task(),
        number_task()
    };
    
    assert_eq!(string_result, "quick");
    assert_eq!(number_result, 42);
}

#[tokio::test]
async fn test_parallel_with_results() {
    let (ok_result, err_result) = parallel! {
        quick_task(),
        error_task()
    };
    
    assert_eq!(ok_result, "quick");
    assert_eq!(err_result, Err("error occurred".to_string()));
}

#[tokio::test]
async fn test_parallel_with_real_functions() {
    let user_id = 123;
    let (posts, followers): (Vec<String>, Vec<String>) = parallel! {
        get_posts(user_id),
        get_followers(user_id)
    };
    
    assert_eq!(posts.len(), 2);
    assert_eq!(followers.len(), 2);
    assert!(posts[0].contains("Post 1"));
    assert!(followers[0].contains("Follower 1"));
} 