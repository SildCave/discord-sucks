mod authentication;
mod refresh;
mod register_user_credential_based;
mod add_user_from_jwt;

use std::sync::Arc;

pub use authentication::AuthenticationState;
pub use refresh::RefreshState;
pub use register_user_credential_based::RegisterUserCredentialBasedState;
pub use add_user_from_jwt::AddUserFromJWTTokenState;


use axum::extract::FromRef;


#[derive(Clone)]
pub struct ApiState {
    pub authentication: Arc<AuthenticationState>,
    pub refresh: Arc<RefreshState>,
    pub register_user_credential_based: Arc<RegisterUserCredentialBasedState>,
    pub add_user_from_jwt: Arc<AddUserFromJWTTokenState>,
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

impl FromRef<ApiState> for Arc<RegisterUserCredentialBasedState> {
    fn from_ref(api_state: &ApiState) -> Arc<RegisterUserCredentialBasedState> {
        api_state.register_user_credential_based.clone()
    }
}

impl FromRef<ApiState> for Arc<AddUserFromJWTTokenState> {
    fn from_ref(api_state: &ApiState) -> Arc<AddUserFromJWTTokenState> {
        api_state.add_user_from_jwt.clone()
    }
}