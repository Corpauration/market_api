

pub trait RouteGroup {
    fn add_to(&self, router: &mut axum::Router);

    fn add_into(&self, mut router: axum::Router) -> axum::Router {
        self.add_to(&mut router);
        router
    }
}

pub trait RouterExt {
    fn routes<Routes: RouteGroup>(&mut self, route_group: Routes);
}
impl RouterExt for axum::Router {
    fn routes<Routes: RouteGroup>(&mut self, route_group: Routes) {
        route_group.add_to(self);
    }
}


