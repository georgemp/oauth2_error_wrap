mod errors;

use errors::Error;
use failure::Fail;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AsyncCodeTokenRequest, ErrorResponse,
};

async fn exchange_code<RE, T>() -> Result<String, Error<RE, T>>
where
    RE: Fail,
    T: ErrorResponse + 'static,
{
    let client = BasicClient::new(
        oauth2::ClientId::new("client_id".to_string()),
        None,
        oauth2::AuthUrl::new("auth_url".to_string()).unwrap(),
        None,
    );
    let oauth_token_response = client
        .exchange_code(oauth2::AuthorizationCode::new("code".to_string()))
        .request_async(async_http_client)
        .await?;

    Ok(format!("{:?}", oauth_token_response))
}
fn main() {
    println!("Hello, world!");
}
