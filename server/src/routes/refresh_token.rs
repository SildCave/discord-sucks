use std::sync::Arc;

use axum::{
    extract::State, http::{
        HeaderMap,
        HeaderValue
    }
};
use axum::http::header::SET_COOKIE;

use axum_extra::TypedHeader;

use headers::Cookie;
use jsonwebtoken::{
    encode, Header
};
use tracing::error;

use crate::{
    auth::{
        extract_token_from_cookie,
        verify_token,
        AuthError,
        ClaimType,
        Claims,
    },
    state::RefreshState
};

use anyhow::Result;


pub async fn refresh_token(
    State(refresh_state): State<Arc<RefreshState>>,
    cookies: TypedHeader<Cookie>,
) -> Result<HeaderMap, AuthError> {
    //let cookie = headers.typed_get::<Cookie>().ok_or(AuthError::MissingCredentials)?;
    let bearer_token = extract_token_from_cookie(
        cookies,
        ClaimType::Refresh
    )?;

    let verification_res = verify_token(
        &bearer_token,
        &refresh_state.jwt_keys,
        Some(ClaimType::Refresh)
    ).await;

    if verification_res.is_err() {
        let verification_error = verification_res.unwrap_err();
        error!("verification error: {:?}", verification_error);
        let error = verification_error.into();
        return Err(error);
    }
    let claims = verification_res.unwrap();

    let user_id = claims.user_id;

    let db_res = refresh_state
        .db_client.get_user_refresh_token_with_caching(
            user_id
        )
        .await;
    if db_res.is_err() {
        let db_error = db_res.unwrap_err();
        error!("db_error: {:?}", db_error);
        let error = db_error.to_auth_error();
        return Err(error);
    }
    let real_refresh_token = db_res.unwrap();

    if real_refresh_token.is_none() {
        return Err(AuthError::InvalidToken);
    }

    if real_refresh_token.unwrap() != bearer_token {
        return Err(AuthError::InvalidToken);
    }

    let claims = Claims::new_access(
        refresh_state.jwt_config.access_key_lifetime_s,
        user_id
    );

    let encoding_res = encode(
        &Header::default(),
        &claims,
        &refresh_state.jwt_keys.encoding
    );
    if encoding_res.is_err() {
        let encoding_error = encoding_res.unwrap_err();
        error!("encoding error: {:?}", encoding_error);
        return Err(AuthError::TokenCreation);
    }

    let token = encoding_res.unwrap();

    let cookie = cookie::Cookie::build(
        (ClaimType::Access.as_str(), format!("Bearer {}", token))
    )
        .max_age(cookie::time::Duration::seconds(
            refresh_state.jwt_config.access_key_lifetime_s
        ))
        //.secure(true)
        .http_only(false)
        .build();

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap()
    );

    Ok(headers)

}