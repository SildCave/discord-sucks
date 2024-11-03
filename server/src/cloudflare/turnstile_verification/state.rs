

#[derive(Debug, Clone)]
pub struct CloudflareTurnstileState {
    pub secret_key: &'static str,
    pub allow_invalid_turnstile: bool,
}

impl CloudflareTurnstileState {
    pub fn new(config: &crate::configuration::Config) -> anyhow::Result<Self> {
        let secret_key = std::fs::read_to_string(&config.cloudflare.turnstile_secret_key_path)?;
        let secret_key = Box::leak(secret_key.into_boxed_str());
        Ok(Self {
            secret_key,
            allow_invalid_turnstile: config.cloudflare.allow_invalid_turnstile,
        })
    }
}