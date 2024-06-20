//// External Imports
use actix::{Actor, Handler, SyncContext, Message};
use actix_web::{
    http::StatusCode,
    error::ResponseError,
    HttpResponse,
};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use failure::Error;
use redis::{RedisResult, Commands}; // for .get() .set() methods
use std::sync::Arc;

//// Internal Imports
use crate::redis_client::{
    start_redis_client,
    exec_redis_command,
    RedisCommand,
    Setex,
    RedisActixError,
};
use crate::AppState;

/////////////////////////
/// DatabaseActor Actor
/////////////////////////

pub struct DatabaseActor {
    pub redis_sync_client: Arc<redis::Client>,
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

// Implement Actor traits for GraphQLExecutor
impl Actor for DatabaseActor {
    type Context = SyncContext<Self>;
}

impl DatabaseActor {
    pub fn new(
        database_actor: Pool<ConnectionManager<PgConnection>>,
    ) -> DatabaseActor {

        DatabaseActor {
            redis_sync_client: start_redis_client(),
            pool: database_actor,
        }
    }

    pub fn get_redis_client(&self) -> Result<redis::Connection, RedisActixError> {
        self.redis_sync_client.clone().get_connection()
            .map_err(|e| RedisActixError::Connection(e.to_string()))
    }

    pub fn setex_to_cache(&self,
        key: String,
        expiry: i32,
        value: String
    ) -> RedisResult<String> {
        match self.redis_sync_client.clone().get_connection() {
            Err(e) => Err(e),
            Ok(mut conn) => redis::cmd("SETEX")
                            .arg(key)
                            .arg(expiry) // TTL in seconds
                            .arg(value)
                            .query(&mut conn),
        }
    }

    pub fn get_from_cache<'a>(&self, key: &'a str) -> RedisResult<String> {
        match self.redis_sync_client.clone().get_connection() {
            Err(e) => Err(e),
            Ok(mut conn) => redis::cmd("GET")
                            .arg(key)
                            .query(&mut conn),
        }
    }
}

////////// GetPool Message Handler ////////////
// Handle messages that tell DatabaseActor to share it's DB pool.
// Allow DatabaseActor to share it's DB pool to other processes
// so other actors do not need to create their own pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetPool {
    Postgres
}
// Actix Message Trait impl for Db queries
impl Message for GetPool {
    type Result = Result<PooledConnection<ConnectionManager<PgConnection>>, GetPoolError>;
}
impl Handler<GetPool> for DatabaseActor {
    type Result = Result<PooledConnection<ConnectionManager<PgConnection>>, GetPoolError>;

    fn handle(&mut self, _msg: GetPool, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.pool.get()
        .map_err(|e| GetPoolError::PoolConnection(e.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, Fail)]
pub enum GetPoolError {
    #[fail(display = "DB Pool Connection error: {}", _0)]
    PoolConnection(String),
}

impl ResponseError for GetPoolError {
    fn error_response(&self) -> HttpResponse {
       match self {
            GetPoolError::PoolConnection(s) => {
                warn!("{}", s);
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
       }
    }
}

/// RedisCommand Handlers for DatabaseActor's sync redis instance.
impl Handler<RedisCommand> for DatabaseActor {
    type Result = Result<String, RedisActixError>;

    fn handle(
        &mut self,
        msg: RedisCommand,
        _ctx: &mut SyncContext<Self>
    ) -> Result<String, RedisActixError> {
        let mut conn = self.redis_sync_client.clone().get_connection()?;
        exec_redis_command(&mut conn, msg)
    }
}
