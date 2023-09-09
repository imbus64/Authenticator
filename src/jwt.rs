use crate::{
    config::{DAYS_VALID, JWT_SECRET},
    Claims,
};
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, DecodingKey, EncodingKey, Header, Validation,
};
use log::*;

// JwtResult is just a predefined error from the jsonwebtoken crate
pub fn token_factory(user: &str) -> JwtResult<String> {
    info!("Issuing JWT token for {}", user);

    let token = encode(
        &Header::default(),
        &Claims {
            sub: user.to_string(),
            iss: "localhost".to_string(),
            aud: "localhost".to_string(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: (chrono::Utc::now() + chrono::Duration::days(DAYS_VALID)).timestamp() as usize,
        },
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .unwrap();

    Ok(token)
}

pub fn validate_token(token: &str) -> JwtResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => {
            info!("Token validated for {}", token_data.claims.sub);
            Ok(token_data.claims)
        }
        Err(e) => {
            error!("Token validation failed: {}", e);
            Err(e)
        }
    }
}
