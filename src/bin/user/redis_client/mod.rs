
pub mod actor;
pub mod errors;

pub use actor::*;
pub use errors::*;

use redis::RedisResult;


pub fn start_redis_server() -> RedisResult<String> {
    // Check if redis-server is already running
    dotenv::dotenv().ok();
    let redis_url = match std::env::var("REDIS_URL") {
        Err(_) => String::from("redis://127.0.0.1:6379"),
        Ok(s) => s,
    };

    match is_redis_live(redis::Client::open(redis_url.as_str())) {
        Ok(res) => Ok(res),
        Err(e) => {
            // Spin up `redis-server` in a background thread.
            std::thread::spawn(|| {
                std::process::Command::new("redis-server").status()
            });
            Err(e)
        }
    }
}

fn is_redis_live(redis_client: RedisResult<redis::Client>) -> RedisResult<String> {
    let res = redis::cmd("SETEX")
        .arg("test").arg(60).arg("ok")
        .query(&mut redis_client?.get_connection()?);
    debug!("<redis-server> exists for dt-user: SETEX test: {:?}", res);
    res
}

pub fn start_redis_client() -> std::sync::Arc<redis::Client> {

    let redis_url = match std::env::var("REDIS_URL") {
        Err(_) => String::from("redis://127.0.0.1"),
        Ok(s) => s,
    };

    std::sync::Arc::new(
        redis::Client::open(redis_url.as_str())
            .expect(&format!("Unable to connect to redis: {}", redis_url)),
    )
}