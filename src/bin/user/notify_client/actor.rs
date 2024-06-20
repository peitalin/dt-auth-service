
use actix::{Actor, Handler, Context, Message, ActorFuture};
use actix_web::Error;
use actix::prelude::{ ResponseActFuture, WrapFuture, AtomicResponse };

use std::sync::Arc;

use futures::{
    future,
    future::Future,
};
use crate::rpc::{
    rpc_notify_user_created,
    rpc_send_welcome_email,
    rpc_send_password_reset_email,
};
use crate::notify_client::{
    NotifyActixError,
};
use crate::AppState;
use crate::endpoints::Endpoint;
use crate::models::errors::{
    RpcError,
    ErrJson,
};




pub struct NotifyActor {
    pub client: Arc<actix_web::client::Client>,
}
impl NotifyActor {
    pub fn new() -> Self {
        Self {
            client: Arc::new(
                actix_web::client::ClientBuilder::new()
                        .header("Content-Type", "application/json")
                        .header("Accept-Encoding", "*")
                        .finish())
        }
    }
}

impl Actor for NotifyActor {
    type Context = Context<Self>;
}

////////// NotifyCommand Message Handler ////////////
/// These are message types to send to
/// execute Notify Commands from other actors

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotifyMessage {
    UserCreated(String),
    SendWelcomeEmail(String),
    SendPasswordResetEmail(
        String, // email,
        String, // resetId,
        chrono::NaiveDateTime, // expiresAt,
    ),
}

impl Message for NotifyMessage {
    type Result = Result<serde_json::Value, NotifyActixError>;
}

type NResponse = Result<serde_json::Value, NotifyActixError>;


impl Handler<NotifyMessage> for NotifyActor {
    // type Result = NResponse;
    type Result = ResponseActFuture<Self, NResponse>;

    fn handle(
        &mut self,
        msg: NotifyMessage,
        _ctx: &mut Context<Self>
    ) -> Self::Result {

        // Arc for client to satisfy 'static lifetime when moving into
        // async { } block
        let ref_client = Arc::clone(&self.client);

        match msg {
            NotifyMessage::UserCreated(user_id) => {
                Box::pin(async move {
                    // Tell the notify service about the new user
                    rpc_notify_user_created(
                        &ref_client,
                        &user_id
                    ).await
                }.into_actor(self))
            },
            NotifyMessage::SendWelcomeEmail(user_id) => {
                Box::pin(async move {
                    // Tell the notify service to send welcome email to user
                    rpc_send_welcome_email(
                        &ref_client,
                        &user_id
                    ).await
                }.into_actor(self))
            },
            NotifyMessage::SendPasswordResetEmail(
                email,
                reset_id,
                expires_at,
            ) => {
                Box::pin(async move {
                    // Tell the notify service to send password reset email
                    rpc_send_password_reset_email(
                        &ref_client,
                        &email,
                        &reset_id,
                        &expires_at
                    ).await
                }.into_actor(self))
            },
        }
    }
}
