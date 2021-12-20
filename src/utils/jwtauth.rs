use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaims {
    username: String,
    email: String,
    iat: usize,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<TokenData<CustomClaims>, Error> {
    let token = decode::<CustomClaims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );
    token
}

pub fn create_token(username: String, email: String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();
    let custom_claims = CustomClaims {
        username: username,
        email: email,
        iat: Utc::now().timestamp() as usize,
        exp: expiration as usize,
    };
    let token = encode(
        &Header::default(),
        &custom_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();
    token
}
