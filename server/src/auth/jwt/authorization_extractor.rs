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
    ClaimType,
    AuthClaims,
    JWTKeys
};


#[async_trait]
impl FromRequestParts<JWTKeys> for AuthClaims
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        jwt_keys: &JWTKeys
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

        let claims: AuthClaims = jwt_keys.verify_token_and_return_claims(
            &bearer_token,
        ).await.map_err(|err| Into::<AuthError>::into(err))?;

        if claims.claim_type != ClaimType::Access {
            return Err(AuthError::InvalidToken);
        }

        // if claims.exp < chrono::Utc::now().timestamp() {
        //     return Err(AuthError::ExpiredToken);
        // }
        Ok(claims)
    }
}