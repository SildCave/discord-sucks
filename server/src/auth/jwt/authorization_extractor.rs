use axum_extra::TypedHeader;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};


use anyhow::Result;

use super::{
    super::AuthError,
    extractors::extract_token_from_cookie,
    verification::verify_token,
    ClaimType,
    Claims,
    JWTKeys
};


#[async_trait]
impl FromRequestParts<JWTKeys> for Claims
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &JWTKeys
    ) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>().await;

        if cookies.is_err() {
            return Err(AuthError::NoToken);
        }
        let bearer_token = extract_token_from_cookie(
            cookies.unwrap(),
            ClaimType::Access
        )?;

        println!("Bearer: {}", bearer_token);

        let claims = verify_token(
            &bearer_token,
            state,
            Some(ClaimType::Access)
        ).await.map_err(|err| Into::<AuthError>::into(err))?;

        // if claims.exp < chrono::Utc::now().timestamp() {
        //     return Err(AuthError::ExpiredToken);
        // }
        Ok(claims)
    }
}