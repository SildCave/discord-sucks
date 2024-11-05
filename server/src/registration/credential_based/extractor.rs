

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

use super::{payload::CredentialBasedRegistrationPayload, RegistrationError};



#[async_trait]
impl<S> FromRequest<S> for CredentialBasedRegistrationPayload
{

    type Rejection = RegistrationError;
    async fn from_request(
        req: Request,
        _state: &S
    ) -> Result<Self, Self::Rejection>
    {

        Err(RegistrationError::InvalidBody)
    }

}
