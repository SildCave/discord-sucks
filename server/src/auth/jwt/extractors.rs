use axum_extra::TypedHeader;
use headers::Cookie;

use crate::auth::AuthError;

use super::ClaimType;



pub fn extract_token_from_cookie(
    cookie: TypedHeader<Cookie>,
    claim_type: ClaimType
) -> Result<String, AuthError> {
    let bearer_token = cookie.get(
        claim_type.as_str()
    );

    if bearer_token.is_none() {
        return Err(AuthError::NoToken);
    }
    let bearer_token = bearer_token.unwrap().to_string().replace("Bearer ", "");
    Ok(bearer_token)

}