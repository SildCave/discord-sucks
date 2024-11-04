use super::{
    state::TurnstileState,
    TurnstileError,
    validation::validate_turnstile_response
};

use std::boxed::Box;

use axum::{
    extract::FromRequest,
    Form,
    Json,
    RequestExt,
    async_trait,
    extract::Request,
    http::header::CONTENT_TYPE
};

use serde::{
    Deserialize,
    Serialize
};

use tracing::error;


#[derive(Debug, Clone, PartialEq)]
pub enum TurnstileResult {
    Allowed,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct TurnstileRequest {
    #[serde(rename = "cf-turnstile-response")]
    cf_turnstile_response: String,
}

#[async_trait]
impl FromRequest<TurnstileState> for TurnstileResult
{

    type Rejection = TurnstileError;
    async fn from_request(
        req: Request,
        state: &TurnstileState
    ) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload): Json<TurnstileRequest> = req.extract().await.map_err(|_| TurnstileError::InvalidBody)?;
                if state.allow_invalid_turnstile {
                    return Ok(TurnstileResult::Allowed);
                }
                get_response_from_payload(
                    state,
                    payload
                ).await?;
            }
            else if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload): Form<TurnstileRequest> = req.extract().await.map_err(|_| TurnstileError::InvalidBody)?;
                if state.allow_invalid_turnstile {
                    return Ok(TurnstileResult::Allowed);
                }
                get_response_from_payload(
                    state,
                    payload
                ).await?;
            }
        }
        Err(TurnstileError::InvalidBody)
    }

}

async fn get_response_from_payload(
    state: &TurnstileState,
    payload: TurnstileRequest
) -> Result<TurnstileResult, TurnstileError> {
    let valid = validate_turnstile_response(
        state,
        &payload.cf_turnstile_response
    ).await;
    if valid.is_err() {
        let error = valid.unwrap_err();
        error!("turnstile validation error: {:?}", error);
        return Err(error);
    }
    if valid.unwrap() == TurnstileResult::Denied {
        return Ok(TurnstileResult::Denied);
    }
    Ok(TurnstileResult::Allowed)
}