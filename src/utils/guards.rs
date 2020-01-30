use crate::utils::jwt::Claims;
use actix_web::dev::RequestHead;

pub fn login_required(request_head: &RequestHead) -> bool {
    let extensions = request_head.extensions();
    let claims = extensions.get::<Claims>();
    match claims {
        Some(claim) => {
            debug!("logged user with email {}", claim.email());
            true
        }
        _ => false,
    }
}

pub fn unlogged_required(request_head: &RequestHead) -> bool {
    let extensions = request_head.extensions();
    let claims = extensions.get::<Claims>();
    match claims {
        Some(claim) => false,
        _ => true,
    }
}
