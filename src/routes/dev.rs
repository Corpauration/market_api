use super::*;
use crate::handlers::{ Provider, };

pub async fn require_superuser_passphrase(request: axum::extract::Request, next: axum::middleware::Next) -> Result<axum::response::Response, axum::http::StatusCode> {
    match cfg!(feature = "allow_superuser_passphrase")
        && std::env::var("ALLOW_SUPERUSER_PASSPHRASE").map(|v| v == "true").unwrap_or(false) {
        true => {
            let auth_header = request.headers().get(axum::http::header::AUTHORIZATION);

            if let Some(auth_header) = auth_header
                && let Ok(auth_str) = auth_header.to_str()
                    && auth_str == "SuperuserPassphrase" {
                        return Ok(next.run(request).await);
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
                extract::{ State as AxumState, Json, },
            },
            crate::handlers::{
                self,
                dev::{ SudoRunSqlRequest, },
            },
        };

        router

            .route("/dev/echo", get(|body: String| async move { body }))

            .route("/dev/now", get(|| async move { chrono::Utc::now().to_rfc3339() }))

            .route("/dev/db", post(|state: AxumState<State>, Json(request): Json<SudoRunSqlRequest<String>>| async move { 
                handlers::dev::handle_sudo_run_sql(state, request).await
            }))

            .route_layer(middleware::from_fn(require_superuser_passphrase))
    }
}
