use super::{
    state::TurnstileState,
    TurnstileError,
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
pub struct TurnstileRequest {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
}


pub trait GetTurnstileCode {
    fn get_turnstile_code(&self) -> String;
}


impl TurnstileState
{

    pub async fn verify_turnstile_from_request(
        &self,
        request: &impl GetTurnstileCode,
    ) -> Result<TurnstileResult, TurnstileError> {

        let turnstile_result = self.get_turnstile_result_from_turnstile_code(
            request.get_turnstile_code()
        ).await?;

        Ok(turnstile_result)

    }

    async fn get_turnstile_result_from_turnstile_code(
        &self,
        code: String
    ) -> Result<TurnstileResult, TurnstileError> {
        let valid = self.validate_cf_turnstile_response(
            &code
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
}

