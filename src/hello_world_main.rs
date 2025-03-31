mod custom_error;

extern crate proc_macro;
use parallel_macro::parallel;
use parallel_macro::timeout;
use parallel_macro::timeout_fallback;
use parallel_macro::timeout_value;
use parallel_macro::timeout_with_result;
use parallel_macro::first;
use parallel_macro::my_test_timeout;
use parallel_macro_core::TimeoutResult;
use std::time::Duration;
use custom_error::CustomError;

async fn get_posts(user_id: u64) -> Vec<String> {
    tokio::time::sleep(Duration::from_millis(1100)).await;
    vec![format!("Post 1 for user {}", user_id), format!("Post 2 for user {}", user_id)]
}

async fn get_followers(user_id: u64) -> Vec<String> {
    tokio::time::sleep(Duration::from_millis(150)).await;
    vec![format!("Follower 1 of user {}", user_id), format!("Follower 2 of user {}", user_id)]
}

async fn get_data() -> i32 {
    tokio::time::sleep(Duration::from_millis(1500)).await;
    100
}

fn process_data(posts: &Vec<String>, followers: &Vec<String>) {
    println!("User has {} posts:", posts.len());
    for post in posts {
        println!("  - {}", post);
    }
    
    println!("User has {} followers:", followers.len());
    for follower in followers {
        println!("  - {}", follower);
    }
}

async fn test_parallel_timeout() {
    let user_id = 123;
    
    // Case #1: Parallel
    let (posts, followers) = parallel! { 
        get_posts(user_id), 
        get_followers(user_id),
    };
    
    process_data(&posts, &followers);

    
    
    // Case #2: Timeout
    let result: Result<i32, String> = timeout!( 1 { 
        get_data()
    } else {
        String::from("too long!")
    }); 
    match result {
        Ok(val) => println!("Sucess: {}", val),
        Err(err) => println!("timeout: {}", err),
    }

    // Case #3: Timeout with value fallback
    let result3: i32 = timeout_fallback!( 1  { 
        get_data()
    } else {
        42
    });
    println!("result3: {}", result3);

    // # Case4: Timeout with value
    let result4  = timeout_value!( 1  { 
        42
    } else {
        String::from("too long #2!")
    });
    
    match result4 {
        Ok(val) => println!("Sucess #4: {}", val),
        Err(err) => println!("timeout #4: {}", err),
    }

    // Case #5: Timeout with parallel
    let result5: Result<(Vec<String>, Vec<String>), String>  = timeout_value!( 1  { 
        parallel! { 
            get_posts(user_id), 
            get_followers(user_id),
        }
    } else {
        String::from("too long #2!")
    });

    match result5 {
        Ok((posts, followers)) => process_data(&posts, &followers),
        Err(err) => println!("timeout2: {}", err),
    }
}

async fn get_data2(t: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(t)).await;
    t
}

async fn get_data_3(t: u64) -> Result<u64, CustomError> {
    tokio::time::sleep(Duration::from_millis(t)).await;

    if (t == 500) {
        return Err(CustomError::not_found(String::from("123")))
    }

    Ok(t)
}

fn do_fallback() -> Result<u64, CustomError> {
    Ok(123)
}

async fn do_fallback_2() -> Result<u64, CustomError> {
    Ok(123)
}

async fn test() -> Result<u64, CustomError> {
    let result = timeout_with_result!(1 {
        get_data_3(1100)
    });

    // If Try is implemented, this would extract the value or return an error
    let value = result?;
    
    Ok(value + 100)
}

#[tokio::main]
async fn main() {
    
    let result = test().await;
    match result {
        Ok(val) => println!("result: {}", val),
        Err(err) => println!("error: {}", err)
    }

    // test_parallel_timeout().await;

    // let r2 = timeout_with_result!(1 {
    //     get_data_3(1100)
    // } else {
    //     println!("Timeout happened?!");
    //     match timeout_with_result!(1 {
    //         get_data_3(1100)
    //     } else {
    //         println!("Timeout happened?!");
    //         Ok(123)
    //         // Err(CustomError{error: String::from("timeout!!")})
    //         // do_fallback();
    //         // do_fallback_2().await
    //     }) {
    //         TimeoutResult::Success(val) => Ok(val),
    //         // function error
    //         TimeoutResult::Error(err) => Err(err),
    //         // timeout error
    //         TimeoutResult::TimedOut => Err(CustomError{error: String::from("should never happen")}),
    //     }
    // });


    // match r2 {
    //     TimeoutResult::Success(val) => println!("success: {}", val),
        
    //     // function error
    //     TimeoutResult::Error(CustomError{error}) => println!("err: {}", error),
        
    //     // timeout error
    //     TimeoutResult::TimedOut => println!("timeout"),
    // }



    // let result: Result<u64, String> = first!({
    //     get_data2(1000),
    //     get_data2(2000)
    // } else {
    //     String::from("everything failed")
    // });
    // match result {
    //     Ok(val) => println!("success: {}", val),
    //     Err(err) => println!("failure: {}", err),
    // }


    // tokio::select! {
    //     _ = get_data2(10) => {
    //         println!("do_stuff_async() completed first")
    //     }
    //     _ = get_data2(100) => {
    //         println!("more_async_work() completed first")
    //     }
    // };

    // println!("Finish");
    
}