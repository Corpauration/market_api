

pub trait AxumRouterStateBound: Clone + Send + Sync + 'static {}
impl<This: Clone + Send + Sync + 'static> AxumRouterStateBound for This {}

pub trait RouteGroup<State: AxumRouterStateBound> {
    fn add_to(self, router: axum::Router<State>) -> axum::Router<State>;
}
pub trait RouteGroupExt<State: AxumRouterStateBound> {
    fn new(self) -> axum::Router<State>;
}
impl<State: AxumRouterStateBound, This: RouteGroup<State>> RouteGroupExt<State> for This {
    fn new(self) -> axum::Router<State> {
        self.add_to(axum::Router::new())
    }
}

pub trait RouterExt<State: AxumRouterStateBound> {
    fn routes<Routes: RouteGroup<State>>(self, route_group: Routes) -> Self;
}
impl<State: AxumRouterStateBound> RouterExt<State> for axum::Router<State> {
    fn routes<Routes: RouteGroup<State>>(self, route_group: Routes) -> Self {
        route_group.add_to(self)
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
