use super::{
    state::TurnstileState,
    TurnstileError,
};

use serde::{
    Deserialize,
    Serialize
};


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
        ).await?;
        if valid == TurnstileResult::Denied {
            return Ok(TurnstileResult::Denied);
        }
        Ok(TurnstileResult::Allowed)
    }
}

