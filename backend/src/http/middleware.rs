use crate::grpc::auth::AuthService;
use std::sync::Arc;
use warp::Filter;

pub fn with_auth(
    auth_service: Arc<AuthService>,
) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("authorization").and_then(move |auth_header: String| {
        let auth_service = auth_service.clone();
        async move {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                match auth_service.verify_token(token) {
                    Ok(claims) => Ok(claims.user_id),
                    Err(_) => Err(warp::reject::custom(AuthError)),
                }
            } else {
                Err(warp::reject::custom(AuthError))
            }
        }
    })
}

#[derive(Debug)]
pub struct AuthError;

impl warp::reject::Reject for AuthError {}
