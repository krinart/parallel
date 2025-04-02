use crate::custom_error::CustomError;
use parallel_macro::timeout_with_result;
use parallel_macro_core::TimeoutResult;
use std::time::Duration;

async fn quick_success_task() -> Result<i32, CustomError> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(42)
}

async fn slow_success_task() -> Result<i32, CustomError> {
    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(100)
}

async fn quick_error_task() -> Result<i32, CustomError> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    Err(CustomError::not_found("test error".to_string()))
}

async fn slow_error_task() -> Result<i32, CustomError> {
    tokio::time::sleep(Duration::from_millis(200)).await;
    Err(CustomError::not_found("slow error".to_string()))
}

async fn timeout_task() -> Result<i32, CustomError> {
    tokio::time::sleep(Duration::from_millis(1500)).await;
    Ok(999)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timeout_with_result_success() {
    let result = timeout_with_result!(1 {
        quick_success_task()
    });
    
    match result {
        TimeoutResult::Success(value) => assert_eq!(value, 42),
        _ => panic!("Expected Success(42)"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timeout_with_result_error() {
    let result = timeout_with_result!(1 {
        quick_error_task()
    });
    
    match result {
        TimeoutResult::Error(err) => {
            match err {
                CustomError::ResourceNotFound(msg) => assert_eq!(msg, "test error"),
                _ => panic!("Expected ResourceNotFound error"),
            }
        },
        _ => panic!("Expected Error"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timeout_with_result_timeout() {
    let result = timeout_with_result!(1 {
        timeout_task()
    });
    
    match result {
        TimeoutResult::TimedOut => (),
        _ => panic!("Expected TimedOut"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timeout_with_result_slow_success() {
    let result = timeout_with_result!(1 {
        slow_success_task()
    });
    
    match result {
        TimeoutResult::Success(value) => assert_eq!(value, 100),
        _ => panic!("Expected Success(100)"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timeout_with_result_slow_error() {
    let result = timeout_with_result!(1 {
        slow_error_task()
    });
    
    match result {
        TimeoutResult::Error(err) => {
            match err {
                CustomError::ResourceNotFound(msg) => assert_eq!(msg, "slow error"),
                _ => panic!("Expected ResourceNotFound error"),
            }
        },
        _ => panic!("Expected Error"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timeout_with_result_with_fallback() {
    let result = timeout_with_result!(1 {
        timeout_task()
    } else {
        Ok(123)
    });
    
    match result {
        TimeoutResult::Success(value) => assert_eq!(value, 123),
        _ => panic!("Expected Success(123) from fallback"),
    }
} 