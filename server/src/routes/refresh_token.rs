use std::{fmt::format, sync::Arc};

use axum::{
    extract::State, http::{request::Parts, HeaderMap, HeaderValue}, Json
};
use headers::{Cookie, HeaderMapExt};
use jsonwebtoken::{
    decode, encode, Header
};

use crate::{auth::{extract_token_from_cookie, verify_token, AuthError, AuthenticationBody, AuthenticationPayload, ClaimType, Claims, JWTKeys}, state::RefreshState};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    RequestPartsExt,
};

use jsonwebtoken::{
    Validation
};
use serde::{Deserialize, Serialize};
use axum::http::header::SET_COOKIE;

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

    verify_token(
        &bearer_token,
        &refresh_state.jwt_keys,
        Some(ClaimType::Refresh)
    ).await.map_err(|err| Into::<AuthError>::into(err))?;

    // Here you must check if refresh token is valid (redis <= postgres)
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    let claims = Claims::new_access(
        refresh_state.jwt_config.access_key_lifetime_s
    );

    let token = encode(&Header::default(), &claims, &refresh_state.jwt_keys.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

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

    println!("{:?}", headers);
    Ok(headers)

}