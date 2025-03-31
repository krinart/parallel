use parallel_macro::parallel;
use parallel_macro::timeout;
use parallel_macro::timeout_fallback;
use std::time::Duration;

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

#[tokio::main]
async fn main() {
    let user_id = 123;
    
    // Run futures in parallel and wait for both to complete
    let (posts, followers) = parallel! { 
        get_posts(user_id), 
        get_followers(user_id),
    };
    
    process_data(&posts, &followers);

    let result: Result<i32, String> = timeout!(1, { 
            get_data().await 
        } else {
            String::from("too long!")
        }
    ); 
    match result {
        Ok(val) => {
            println!("Sucess: {}", val);
        },
        Err(err) => {
            println!("timeout: {}", err);
        }
    }

    let result2: Result<(Vec<String>, Vec<String>), String>  = timeout!(1, { 
        parallel! { 
            get_posts(user_id), 
            get_followers(user_id),
        }
    } else {
        String::from("too long #2!")
    });

    
    match result2 {
        Ok((posts, followers)) => {
            process_data(&posts, &followers);
        },
        Err(err) => {
            println!("timeout2: {}", err);
        }
    }


    let result3: i32 = timeout_fallback!(1, { 
        get_data().await 
    } else {
        42
    });
    println!("result3: {}", result3);
}