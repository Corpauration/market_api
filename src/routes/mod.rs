

pub trait RouteGroup {
    fn add_to(&self, router: axum::Router) -> axum::Router;

}

pub trait RouterExt {
    fn routes<Routes: RouteGroup>(self, route_group: Routes) -> Self;
}
impl RouterExt for axum::Router {
    fn routes<Routes: RouteGroup>(self, route_group: Routes) -> Self {
        route_group.add_to(self)
    }
}


