use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::domain::identity::user::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,            // User ID
    pub org_id: Uuid,         // Organization ID for RLS
    pub role: UserRole,
    pub exp: i64,             // Expiration timestamp
    pub iat: i64,             // Issued at timestamp
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn create_token(&self, user_id: Uuid, org_id: Uuid, role: UserRole) -> anyhow::Result<String> {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        let claims = Claims {
            sub: user_id,
            org_id,
            role,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to create token: {}", e))
    }

    pub fn verify_token(&self, token: &str) -> anyhow::Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| anyhow::anyhow!("Invalid token: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_flow() {
        let service = JwtService::new("secret");
        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let role = UserRole::Admin;

        let token = service.create_token(user_id, org_id, role).unwrap();
        let claims = service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.org_id, org_id);
        assert_eq!(claims.role, role);
    }
}
