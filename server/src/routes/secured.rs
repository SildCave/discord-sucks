use crate::auth::Claims;

pub async fn secured(claims: Claims) -> &'static str {
    "Secured!"
}