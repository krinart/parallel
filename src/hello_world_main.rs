use parallel_macro::parallel;
use parallel_macro::timeout;
use std::time::Duration;

async fn get_posts(user_id: u64) -> Vec<String> {
    tokio::time::sleep(Duration::from_millis(100)).await;
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

#[tokio::main]
async fn main() {
    let user_id = 123;
    
    // Run futures in parallel and wait for both to complete
    let (posts, followers, followers2) = parallel! { 
        get_posts(user_id), 
        get_followers(user_id),
        get_followers(user_id),
    };
    
    println!("User has {} posts:", posts.len());
    for post in posts {
        println!("  - {}", post);
    }
    
    println!("User has {} followers:", followers.len());
    for follower in followers {
        println!("  - {}", follower);
    }

    println!("User has {} followers:", followers2.len());
    for follower in followers2 {
        println!("  - {}", follower);
    }

    // let (posts2, followers2) = parallel! { 
    //     get_posts(user_id), 
    //     get_followers(user_id),
    // };

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
        Err(val) => {
            println!("timeout: {}", val);
        }
    }


}