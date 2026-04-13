use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    async_trait,
    RequestPartsExt,
};
use crate::error::AppError;
use crate::infrastructure::auth::jwt::{Claims, JwtService};
use axum_extra::TypedHeader;
use headers::authorization::Bearer;
use headers::Authorization;

pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Get JwtService (In production, use state/extensions)
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
        let jwt_service = JwtService::new(&jwt_secret);

        // 2. Extract Bearer token using TypedHeader
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        // 3. Verify token
        let claims = jwt_service.verify_token(bearer.token())
            .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthenticatedUser(claims))
    }
}
