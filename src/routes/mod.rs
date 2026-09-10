

pub trait AxumRouterStateBound: Clone + Send + Sync + 'static {}
impl<This: Clone + Send + Sync + 'static> AxumRouterStateBound for This {}

pub trait RouterBundle<State: AxumRouterStateBound> {
    fn apply_to(self, router: axum::Router<State>) -> axum::Router<State>;
}
pub trait RouterBundleExt<State: AxumRouterStateBound> {
    fn new(self) -> axum::Router<State>;

    fn merge_into(self, router: axum::Router<State>) -> axum::Router<State>;
}
impl<State: AxumRouterStateBound, This: RouterBundle<State>> RouterBundleExt<State> for This {
    fn new(self) -> axum::Router<State> {
        self.apply_to(axum::Router::new())
    }

    fn merge_into(self, router: axum::Router<State>) -> axum::Router<State> {
        router.merge(self.new())
    }
}

pub trait RouterExt<State: AxumRouterStateBound> {
    fn apply_bundle<Bundle: RouterBundle<State>>(self, bundle: Bundle) -> Self;

    fn merge_bundle<Bundle: RouterBundle<State>>(self, bundle: Bundle) -> Self;
}
impl<State: AxumRouterStateBound> RouterExt<State> for axum::Router<State> {
    fn apply_bundle<Bundle: RouterBundle<State>>(self, bundle: Bundle) -> Self {
        bundle.apply_to(self)
    }

    fn merge_bundle<Bundle: RouterBundle<State>>(self, bundle: Bundle) -> Self {
        bundle.merge_into(self)
    }
}

#[derive(Debug)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

impl<'l> crate::handlers::Provider<&'l sqlx::PgPool> for &'l AppState {
    fn provide(self) -> &'l sqlx::PgPool {
        &self.db_pool
    }
}

pub mod dev;
