use super::{
    TurnstileState,
    TurnstileError,
    TurnstileResult
};

use reqwest::{
    Client,
    multipart::Form,
};

use tracing::error;

impl TurnstileState {
    pub async fn validate_cf_turnstile_response(
        &self,
        cf_turnstile_response: &String,
    ) -> Result<TurnstileResult, TurnstileError> {
        let client = Client::new();
        let form = Form::new()
            .text("response", cf_turnstile_response.clone())
            .text("secret", self.secret_key);
        let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
        let response = client.post(url)
            .multipart(form)
            .send()
            .await;
        if response.is_err() {
            error!("turnstile request error: {:?}", response.unwrap_err());
            return Err(TurnstileError::InternalError("1500"));
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            error!("turnstile request failed, response: {:?}", response);
            return Err(TurnstileError::InternalError("1501"));
        }
    
        let response = response.text().await;
        if response.is_err() {
            error!("response error: {:?}", response.unwrap_err());
            return Err(TurnstileError::InternalError("1502"));
        }
    
        let response_json = serde_json::from_str::<serde_json::Value>(&response.unwrap());
        if response_json.is_err() {
            error!("response json error: {:?}", response_json.unwrap_err());
            return Err(TurnstileError::InternalError("1503"));
        }
        let response_json = response_json.unwrap();
    
        let error_codes = response_json["error-codes"].as_array();
    
        if error_codes.is_some() {
            //error!("error-codes is some");
            let error_codes = error_codes.unwrap();
            if error_codes.len() > 0 {
                println!("{:?}", error_codes);
                let error_code = error_codes[0].as_str().unwrap();
                if error_code == "invalid-input-secret" {
                    error!("invalid server secret");
                    return Err(TurnstileError::InternalError("1505"));
                }
                if error_code == "invalid-input-response" {
                    error!("invalid response");
                    return Err(TurnstileError::InternalError("1506"));
                }
                if error_code == "timeout-or-duplicate" {
                    //error!("timeout or duplicate");
                    return Ok(TurnstileResult::Denied);
                }
            }
        }
    
        let success = response_json["success"].as_bool();
        if success.is_none() {
            error!("success is none");
            return Err(TurnstileError::InternalError("1504"));
        }
    
        if !success.unwrap() {
            error!("success is false");
            return Ok(TurnstileResult::Denied);
        }
    
        Ok(TurnstileResult::Allowed)
    }
}
