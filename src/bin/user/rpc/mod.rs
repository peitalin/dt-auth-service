use actix_web::{HttpResponse, HttpRequest, Error};
use dt::utils::dates::from_datetimestr_to_naivedatetime;
use dt::utils::dates::from_datetimestr_to_option_naivedatetime;

/// Remote Procedure Calls with REST
use crate::AppState;
use crate::endpoints::Endpoint;
use crate::models::errors::{
    RpcError,
    ErrJson,
};
use futures::future::Future;
use crate::models::{
    CustomerStripeCompact,
    UpdateUserProfile,
    PayoutMethod,
};
use crate::db::updateUser;
use crate::models::auth::{ CreateUserForm };
use crate::notify_client::errors::NotifyActixError;



pub async fn rpc_test_handler(
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let mut response = AppState::from(&req).http_client
                    .get(Endpoint::Payment("/test").as_url())
                    .send()
                    .await?;

    let bytes = response.body().await?;

    let payment_msg = std::str::from_utf8(&bytes)?;

    Ok(HttpResponse::Ok()
        .content_type("application_json")
        .json(json!({
            "status": "OK",
            "message": "Test response for dt-user service",
            "message2": payment_msg,
        })))

}


pub async fn rpc_create_stripe_customer(
    client: &actix_web::client::Client,
    email: &str,
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
) -> Result<CustomerStripeCompact, Error> {

    let mut response = client
                    .post(Endpoint::Payment("/stripe/customer/create").as_url())
                    .send_json(&json!({
                        "email": email,
                        "name": format_name(first_name, last_name),
                        "description": username,
                        "expand": vec![] as Vec<String>,
                        // workaround, until we deploy ne payment-service
                        // with expand args removed
                    }))
                    .await?;

    debug!("raw response: {:?}", response);
    let bytes = response.body().await?;

    serde_json::from_slice::<CustomerStripeCompact>(&bytes)
        .map_err(|e| Error::from(RpcError::Customer(errJson!(e))))
}

pub fn format_name(
    first_name: Option<String>,
    last_name: Option<String>
) -> String {
    match (first_name, last_name) {
        (Some(f), Some(l)) => format!("{} {}", f, l),
        (Some(f), None) => format!("{}", f),
        (None, Some(l)) => format!("{}", l),
        (None, None) => String::from(""),
    }
}


pub async fn rpc_attach_payment_method(
    client: &actix_web::client::Client,
    payment_method_id: &str,
    customer_id: &str,
) -> Result<serde_json::Value, Error> {

    let url = format!("/stripe/paymentMethod/attach?id={}", payment_method_id);

    let mut response = client
                    .post(Endpoint::Payment(&url).as_url())
                    .send_json(&json!({
                        "customer": customer_id,
                    }))
                    .await?;

    let bytes = response.body().await?;

    let attach_payment_response = std::str::from_utf8(&bytes)
        .map(String::from)
        .map_err(|e| Error::from(RpcError::Customer(errJson!(e))))?;

    Ok(json!({
        "response": attach_payment_response,
        "endpoint": Endpoint::Payment("/stripe/paymentMethod/attach").as_url(),
    }))
}


pub async fn rpc_detach_payment_method(
    client: &actix_web::client::Client,
    payment_method_id: &String,
    user_id: &String,
) -> Result<serde_json::Value, Error> {

    let mut response = client
                    .post(Endpoint::Payment("/paymentMethods/detach/delete").as_url())
                    .send_json(&json!({
                        "userId": user_id,
                        "paymentMethodId": payment_method_id
                    }))
                    .await?;

    let bytes = response.body().await?;

    let detach_payment_response = std::str::from_utf8(&bytes)
        .map(String::from)
        .map_err(|e| Error::from(RpcError::Payment(errJson!(e))))?;

    Ok(json!({
        "paymentMethods": detach_payment_response,
        "endpoint": Endpoint::Payment("/stripe/paymentMethod/detach").as_url(),
    }))
}


pub async fn rpc_list_payment_methods(
    client: &actix_web::client::Client,
    customer_id: &str,
) -> Result<serde_json::Value, Error> {

    let mut response = client
                    .post(Endpoint::Payment("/stripe/paymentMethod/list").as_url())
                    .send_json(&json!({
                        "customer": customer_id,
                        "type": "card",
                    }))
                    .await?;

    let bytes = response.body().await?;

    let detach_payment_response = std::str::from_utf8(&bytes)
        .map(String::from)
        .map_err(|e| Error::from(RpcError::Customer(errJson!(e))))?;

    Ok(json!({
        "response": detach_payment_response,
        "endpoint": Endpoint::Payment("/stripe/paymentMethod/list").as_url(),
    }))
}


pub async fn rpc_setup_intent_create(
    client: &actix_web::client::Client,
    user_id: &str,
    payment_method_id: &str,
    customer_id: &str,
) -> Result<serde_json::Value, Error> {

    let url = format!("/stripe/setupIntent/create?user_id={}", user_id);
    debug!("requesting: {:?}", &url);

    let mut response = client
                    .post(Endpoint::Payment(&url).as_url())
                    .send_json(&json!({
                        "confirm": true,
                        "payment_method": payment_method_id,
                        "customer": customer_id,
                    }))
                    .await?;

    let bytes = response.body().await?;


    let setup_intent_create_response = std::str::from_utf8(&bytes)
        .map(String::from)
        .map_err(|e| Error::from(RpcError::Customer(errJson!(e))))?;

    Ok(json!({
        "response": setup_intent_create_response,
        "endpoint": Endpoint::Payment("/stripe/setupIntent/create").as_url(),
    }))
}


pub async fn rpc_set_payout_method(
    client: &actix_web::client::Client,
    store_id: &str,
    payout_processor: &str,
    payout_type: Option<String>,
    payout_email: Option<String>,
    payout_processor_id: Option<String>,
) -> Result<PayoutMethod, Error> {

    let url = format!("/payoutMethod/write");

    let mut response = client
                    .post(Endpoint::Payment(&url).as_url())
                    .send_json(&json!({
                        "storeId": store_id.clone(),
                        "payoutProcessor": payout_processor,
                        "payoutType": payout_type,
                        "payoutEmail": payout_email,
                        "payoutProcessorId": payout_processor_id,
                    }))
                    .await?;

    let bytes = response.body().await?;

    serde_json::from_slice::<PayoutMethod>(&bytes)
        .map_err(Error::from)
}


pub async fn rpc_read_payout_method(
    client: &actix_web::client::Client,
    payout_method_id: &str,
) -> Result<PayoutMethod, Error> {

    let url = format!("/payoutMethod/read?payout_method_id={}", payout_method_id);

    let mut response = client.get(Endpoint::Payment(&url).as_url())
                    .send()
                    .await?;

    let bytes = response.body().await?;

    serde_json::from_slice::<PayoutMethod>(&bytes)
        .map_err(Error::from)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserShoppingResponse {
    pub success: bool
}

pub async fn rpc_delete_user_shopping(
  client: &actix_web::client::Client,
  user_id: &str
) -> Result<DeleteUserShoppingResponse, Error> {

    let url = format!("/user/{}", user_id);

    let mut response = client.delete(Endpoint::Shopping(&url).as_url())
                    .send()
                    .await?;

    let bytes = response.body().await?;

    serde_json::from_slice::<DeleteUserShoppingResponse>(&bytes)
        .map_err(Error::from)
}



pub async fn rpc_notify_user_created(
    client: &actix_web::client::Client,
    user_id: &str,
) -> Result<serde_json::Value, NotifyActixError> {

    let route = "/internal/account/created";
    debug!("requesting endpoint: {}", route);

    let mut response = client
                    .post(Endpoint::Notify(&route).as_url())
                    .send_json(&json!({
                        "userId": user_id,
                    }))
                    .await
                    .map_err(|e| NotifyActixError::UserCreated(errJson!(e)))?;

    response.json().await
        .map_err(|e| NotifyActixError::UserCreated(errJson!(e)))
}



pub async fn rpc_send_welcome_email(
    client: &actix_web::client::Client,
    user_id: &str,
) -> Result<serde_json::Value, NotifyActixError> {

    let route = "/email/welcome";
    debug!("requesting endpoint: {}", route);

    let mut response = client
                    .post(Endpoint::Notify(&route).as_url())
                    .send_json(&json!({
                        "userId": user_id,
                    }))
                    .await
                    .map_err(|e| NotifyActixError::WelcomeEmail(errJson!(e)))?;

    response.json().await
        .map_err(|e| NotifyActixError::UserCreated(errJson!(e)))
}



pub async fn rpc_send_password_reset_email(
    client: &actix_web::client::Client,
    email: &str,
    reset_id: &str,
    expires_at: &chrono::NaiveDateTime,
) -> Result<serde_json::Value, NotifyActixError> {

    let route = "/email/password-reset";
    debug!("requesting endpoint: {}", route);

    let expires_at_rpc = expires_at
                            .format("%Y-%m-%dT%H:%M:%S")
                            .to_string();
    debug!("sending to notify-service: expires_at: {:?}", expires_at_rpc);

    let mut response = client
                    .post(Endpoint::Notify(&route).as_url())
                    .send_json(&json!({
                        "email": email,
                        "resetId": reset_id,
                        "expiresAt": expires_at_rpc
                    }))
                    .await
                    .map_err(|e| NotifyActixError::PasswordResetEmail(errJson!(e)))?;

    debug!("response: {:?}", response);

    response.json().await
        .map_err(|e| NotifyActixError::UserCreated(errJson!(e)))
}
