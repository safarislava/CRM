use crate::jwt::jwt_secret;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    sub: Uuid,
    jti: Uuid,
    typ: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: Uuid, jti: Uuid, typ: String, exp: usize) -> Self {
        Self { sub, jti, typ, exp }
    }

    pub fn from(token: &str) -> Option<Self> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret().as_bytes()),
            &Validation::default(),
        )
        .ok()
        .map(|d| d.claims)
    }

    pub fn sub(&self) -> Uuid {
        self.sub
    }

    pub fn jti(&self) -> Uuid {
        self.jti
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{EncodingKey, Header, encode};

    #[test]
    fn decodes_valid_jwt_token() {
        let sub = Uuid::new_v4();
        let jti = Uuid::new_v4();
        let claims = Claims::new(sub, jti, "access".to_string(), 9999999999);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )
        .unwrap();
        assert_eq!(Claims::from(&token), Some(claims));
    }

    #[test]
    fn returns_none_for_invalid_token() {
        assert_eq!(Claims::from("invalid.jwt.token"), None);
    }
}
