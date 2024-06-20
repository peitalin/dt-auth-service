
use actix::{Actor, Handler, Context, Message};
use crate::redis_client::{
    start_redis_client,
    RedisActixError,
};
use std::sync::Arc;

pub struct RedisActor {
    pub client: Arc<redis::Client>,
}

impl RedisActor {
    pub fn new() -> Self {
        Self { client:  start_redis_client() }
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

////////// RedisCommand Message Handler ////////////
/// These are message types to send to DatabaseActor
/// to execute Redis Commands from other actors

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RedisCommand {
    Setex(Setex),
    Set(String, String),
    Get(String),
    Del(String),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Setex {
    pub key: String,
    pub ttl: i32, // TTL in seconds
    pub value: String,
}

impl Message for RedisCommand {
    type Result = Result<String, RedisActixError>;
}

impl Handler<RedisCommand> for RedisActor {
    type Result = Result<String, RedisActixError>;

    fn handle(
        &mut self,
        msg: RedisCommand,
        _ctx: &mut Context<Self>
    ) -> Result<String, RedisActixError> {
        let mut conn = self.client.clone().get_connection()?;
        exec_redis_command(&mut conn, msg)
    }
}

pub fn exec_redis_command(
    conn: &mut redis::Connection,
    msg: RedisCommand
) -> Result<String, RedisActixError> {
    let res = match msg {
        RedisCommand::Get(key) => {
            redis::cmd("GET")
                .arg(key)
                .query(conn)
        },
        RedisCommand::Del(key) => {
            redis::cmd("DEL")
                .arg(key)
                .query(conn)
        },
        RedisCommand::Set(key, value) => {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query(conn)
        },
        RedisCommand::Setex(setex) => {
            redis::cmd("SETEX")
                .arg(setex.key)
                .arg(setex.ttl)
                .arg(setex.value)
                .query(conn)
        }
    };
    res.map_err(RedisActixError::from)
}