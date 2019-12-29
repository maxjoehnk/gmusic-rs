use std::io;

use failure::Error;
use oauth2::{AsyncCodeTokenRequest, AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;

use crate::GoogleMusicApiClient;

static SCOPE: &str = "https://www.googleapis.com/auth/skyjam";
static REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

pub(crate) async fn perform_oauth(client: &GoogleMusicApiClient) -> Result<BasicTokenResponse, Error> {
    let client = BasicClient::new(
        ClientId::new(client.id.clone()),
        Some(ClientSecret::new(client.secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
        Some(TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?),
    )
        .set_redirect_url(RedirectUrl::new(REDIRECT_URI.to_string())?);

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, _) = client.authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(SCOPE.to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    let mut code = String::new();
    io::stdin().read_line(&mut code)?;

    let code = AuthorizationCode::new(code);

    let token = client.exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await?;

    Ok(token)
}