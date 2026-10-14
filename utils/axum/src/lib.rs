
/// Equivalent to usual requirements for axum router states.
pub trait AxumRouterState: Clone + Send + Sync + 'static {}
impl<State: Clone + Send + Sync + 'static> AxumRouterState for State {}

/// Capability to bulk configure a router.
/// Usually registering a set of routes.
pub trait RouteBundle<State: AxumRouterState = ()>
    : Sized
{
    /// Rawly apply this bundle to a router.
    fn apply_to(self, router: axum::Router<State>) -> axum::Router<State>;

    /// Apply this bundle to a new router.
    fn into_router(self) -> axum::Router<State> {
        self.apply_to(axum::Router::new())
    }

    /// Merge this bundle with an existing router.
    fn merge_with(self, router: axum::Router<State>) -> axum::Router<State> {
        router.merge(self.into_router())
    }

    /// Nest this bundle into an existing router.
    fn nest_in(self, path: &str, router: axum::Router<State>) -> axum::Router<State> {
        router.nest(path, self.into_router())
    }
}

/// Extension trait for axum routers to support route bundles.
pub trait RouterExt<State: AxumRouterState>
{
    /// Rawly apply a route bundle to this router.
    fn apply_bundle(self, bundle: impl RouteBundle<State>) -> Self;

    /// Merge a route bundle with this router.
    fn merge_bundle(self, bundle: impl RouteBundle<State>) -> Self;

    /// Nest a route bundle into this router.
    fn nest_bundle(self, path: &str, bundle: impl RouteBundle<State>) -> Self;
}
impl<State: AxumRouterState> RouterExt<State> for axum::Router<State>
{
    fn apply_bundle(self, bundle: impl RouteBundle<State>) -> Self {
        bundle.apply_to(self)
    }

    fn merge_bundle(self, bundle: impl RouteBundle<State>) -> Self {
        bundle.merge_with(self)
    }

    fn nest_bundle(self, path: &str, bundle: impl RouteBundle<State>) -> Self {
        bundle.nest_in(path, self)
    }
}
