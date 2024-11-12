use super::{
    TurnstileState,
    TurnstileError,
    TurnstileResult
};

use reqwest::{
    Client,
    multipart::Form,
};


impl TurnstileState {
    // No logging in this function
    pub async fn validate_cf_turnstile_response(
        &self,
        cf_turnstile_response: &String,
    ) -> Result<TurnstileResult, TurnstileError> {
        if self.allow_invalid_turnstile {
            return Ok(TurnstileResult::Allowed);
        }
        let form = Form::new()
            .text("response", cf_turnstile_response.clone())
            .text("secret", self.secret_key);

        let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

        // imposter

        let response = self.reqwest_client.post(url)
            .multipart(form)
            .send()
            .await?;
        //
        //return Ok(TurnstileResult::Denied);

        let response_status = response.status();
        let response_text = response.text().await?;

        if !response_status.is_success() {
            return Err(TurnstileError::RequestFailed(response_text));
        }

        let response_json = serde_json::from_str::<serde_json::Value>(
            &response_text
        )?;


        let error_codes = response_json["error-codes"].as_array();

        if let Some(error_codes) = error_codes {
            if error_codes.len() > 0 {
                let error_code = error_codes[0].as_str().unwrap();
                if error_code == "invalid-input-secret" {
                    return Err(TurnstileError::InvalidInputSecret);
                }
                if error_code == "invalid-input-response" {
                    return Err(TurnstileError::InvalidInputResponse);
                }
                if error_code == "timeout-or-duplicate" {
                    return Ok(TurnstileResult::Denied);
                }
            }
        }

        let success = response_json["success"].as_bool().ok_or(
            TurnstileError::SuccessFieldNotFound
        )?;

        if !success {
            return Ok(TurnstileResult::Denied);
        }
        
        Ok(TurnstileResult::Allowed)
    }
}
