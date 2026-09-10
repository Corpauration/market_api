use super::*;
use crate::handlers::{ Provider, };

pub async fn require_superuser_passphrase(request: axum::extract::Request, next: axum::middleware::Next) -> Result<axum::response::Response, axum::http::StatusCode> {
    match cfg!(feature = "allow_superuser_passphrase")
        && std::env::var("ALLOW_SUPERUSER_PASSPHRASE").map(|v| v == "true").unwrap_or(false) {
        true => {
            let auth_header = request.headers().get(axum::http::header::AUTHORIZATION);

            if let Some(auth_header) = auth_header {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str == "SuperuserPassphrase" {
                        return Ok(next.run(request).await);
                    }
                }
            }

            Err(axum::http::StatusCode::UNAUTHORIZED)
        },
        false => Err(axum::http::StatusCode::FORBIDDEN),
    }
}

pub struct DevRoutes;

impl<State: AxumRouterStateBound> RouterBundle<State> for DevRoutes
where
    State: Provider<&'static sqlx::PgPool>,
{
    fn apply_to(self, router: axum::Router<State>) -> axum::Router<State> {
        #[allow(unused_imports)]
        use {
            axum::{
                routing::{ get, post, },
                middleware,
            },
            crate::handlers,
        };

        router
            .route("/dev/db", post(handlers::dev::handle_sudo_run_sql_from_json::<'static, State, &'static sqlx::PgPool>))
            .route_layer(middleware::from_fn(require_superuser_passphrase))
    }
}
