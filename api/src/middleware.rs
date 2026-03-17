use uuid::Uuid;

use crate::auth::Claims;

#[derive(Clone, Debug)]
pub struct TenantContext(pub Uuid);

pub fn get_claims(extensions: &axum::http::Extensions) -> Option<&Claims> {
    extensions.get::<Claims>()
}
