use std::io;

use failure::Error;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope};

static SCOPE: &str = "https://www.googleapis.com/auth/skyjam";

pub(crate) fn perform_oauth(client: &BasicClient) -> Result<BasicTokenResponse, Error> {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, _) = client
        .authorize_url(CsrfToken::new_random)
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

    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request(http_client)?;

    Ok(token)
}
