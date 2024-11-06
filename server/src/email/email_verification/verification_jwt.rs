use crate::{auth::{JWTKeys, VerificationError}, registration::UserRegistrationFormJWT};

impl UserRegistrationFormJWT {
    pub fn into_jwt_token(
        &self,
        keys: &JWTKeys,
    ) -> Result<String, VerificationError> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &keys.encoding,
        )?;
        Ok(token)
    }
}