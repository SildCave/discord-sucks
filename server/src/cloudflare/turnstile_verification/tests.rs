// https://developers.cloudflare.com/turnstile/troubleshooting/testing/


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        cloudflare::{
            turnstile_verification::TurnstileError,
            TurnstileResult,
            TurnstileState
        },
        configuration::Config
    };

    use pretty_assertions::assert_eq;

    pub fn get_config() -> Config {
        let mut cfg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cfg_path.push("../configuration/server/config.toml");
        let mut cfg = Config::from_file(cfg_path).unwrap();
        let mut turnstile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        turnstile_path.push("..");
        turnstile_path.push(&cfg.cloudflare.turnstile_secret_key_path);
        cfg.cloudflare.turnstile_secret_key_path = turnstile_path.to_str().unwrap().to_string();

        cfg
    }

    async fn get_state(
        key: String
    ) -> TurnstileState {
        let config = get_config();
        let mut state = TurnstileState::new(
            &config
        ).unwrap();
        state.secret_key = Box::leak(key.into_boxed_str());
        state

    }

    #[tokio::test]
    async fn test_get_state() {
        let key = "test_key".to_string();
        get_state(key).await;
    }

    #[tokio::test]
    async fn test_working_turnstile_test_key() {
        let key = "1x0000000000000000000000000000000AA".to_string();
        let state = get_state(key).await;
        let res = state.validate_cf_turnstile_response(
            &"XXXX.DUMMY.TOKEN.XXXX".to_string()
        ).await;
        assert_eq!(res.is_ok(), true);

        let res = res.unwrap();
        assert_eq!(res, TurnstileResult::Allowed);
    }

    #[tokio::test]
    async fn test_always_failing_turnstile_test_key() {
        let key = "2x0000000000000000000000000000000AA".to_string();
        let state = get_state(key).await;
        let res = state.validate_cf_turnstile_response(
            &"XXXX.DUMMY.TOKEN.XXXX".to_string()
        ).await;
        assert_eq!(res.is_ok(), false);
        let error = res.unwrap_err();
        assert_eq!(error, TurnstileError::InternalError("1506"));
    }

    #[tokio::test]
    async fn test_invalid_turnstile_test_key() {
        let key = "3x0000000000000000000000000000000AA".to_string();
        let state = get_state(key).await;
        let res = state.validate_cf_turnstile_response(
            &"XXXX.DUMMY.TOKEN.XXXX".to_string()
        ).await;
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), TurnstileResult::Denied);
    }
}