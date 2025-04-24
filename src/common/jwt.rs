use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{env, fmt::Display};
use utoipa::ToSchema;

use super::error::AppError;

/// JWT_SECRET_KEY is the environment variable that holds the secret key for JWT encoding and decoding.
/// It is loaded from the environment variables using the dotenv crate.
/// The secret key is used to sign the JWT tokens and should be kept secret.
pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenvy::dotenv().ok();

    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    Keys::new(secret.as_bytes())
});

/// Keys is a struct that holds the encoding and decoding keys for JWT.
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

/// The Keys struct is used to create the encoding and decoding keys for JWT.
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Claims is a struct that represents the claims in the JWT token.
/// It contains the subject (user ID), expiration time, and issued at time.
/// The `sub` field is the user ID, `exp` is the expiration time, and `iat` is the issued at time.
/// The `Claims` struct is used to encode and decode the JWT tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// The Claims struct implements the `Display` trait for easy printing.
/// It formats the claims as a string, showing the user ID.
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user_id: {}", self.sub)
    }
}

/// The Default trait is implemented for the Claims struct.
/// It sets the default values for the claims.
impl Default for Claims {
    fn default() -> Self {
        let now = Utc::now();
        let expire: Duration = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        Claims {
            sub: String::new(),
            exp,
            iat,
        }
    }
}

/// AuthBody is a struct that represents the authentication body.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

/// The AuthBody struct is used to create a new instance of the authentication body.
/// It takes an access token as a parameter and sets the token type to "Bearer".
impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// AuthPayload is a struct that represents the authentication payload.
/// It contains the client ID and client secret.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

/// make_jwt_token is a function that creates a JWT token.
/// It takes a user ID as a parameter and returns a Result with the JWT token or an error.
pub fn make_jwt_token(user_id: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub: user_id.to_string(),
        ..Default::default()
    };
    encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AppError::TokenCreation)
}

/// Middleware to validate JWT tokens.
/// If the token is valid, the request proceeds; otherwise, a 401 Unauthorized is returned.
pub async fn jwt_auth<B>(mut req: Request<B>, next: Next) -> Result<Response, Response>
where
    B: Send + Into<axum::body::Body>,
{
    // Try to extract and trim the token in one go.
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .ok_or_else(|| AppError::InvalidToken.into_response())?;

    // Validate and decode the token.
    let token_data =
        decode::<Claims>(token, &KEYS.decoding, &Validation::default()).map_err(|err| {
            tracing::error!("Error decoding token: {:?}", err);
            AppError::InvalidToken.into_response()
        })?;

    // Insert the decoded claims into the request extensions.
    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req.map(Into::into)).await)
}
