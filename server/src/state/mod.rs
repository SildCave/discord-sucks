mod authentication;
mod refresh;

use std::sync::Arc;

pub use authentication::AuthenticationState;
pub use refresh::RefreshState;


use axum::extract::FromRef;


#[derive(Clone)]
pub struct ApiState {
    pub authentication: Arc<AuthenticationState>,
    pub refresh: Arc<RefreshState>,
}

impl FromRef<ApiState> for Arc<AuthenticationState> {
    fn from_ref(api_state: &ApiState) -> Arc<AuthenticationState> {
        api_state.authentication.clone()
    }
}

impl FromRef<ApiState> for Arc<RefreshState> {
    fn from_ref(api_state: &ApiState) -> Arc<RefreshState> {
        api_state.refresh.clone()
    }
}
