use crate::core::method::Method;
use crate::core::request::Request;
use std::collections::HashMap;

pub type RouteActions = Result<u8, std::io::Error>;
pub type RouteHandler = fn(&mut Request) -> RouteActions;

pub type MethodMap = HashMap<String, RouteHandler>;
pub type RoutesMap = HashMap<Method, MethodMap>;

/// Routes
///
/// A collection of route handlers.
///
pub struct Routes {
    routes: RoutesMap,
}

unsafe impl Send for Routes {}
unsafe impl Sync for Routes {}

impl Routes {
    pub fn new() -> Self {
        Routes {
            routes: HashMap::new(),
        }
    }

    pub fn find(&self, request: &mut Request) -> Option<RouteHandler> {
        match self.routes.get(&Method::from(&request.method)) {
            Some(method_map) => match method_map.get(&request.uri) {
                Some(handler) => Some(*handler),
                None => match method_map.get("*") {
                    Some(handler) => Some(*handler),
                    None => None,
                },
            },
            None => match self.routes.get(&Method::GET) {
                Some(method_map) => match method_map.get("*") {
                    Some(handler) => Some(*handler),
                    None => None,
                },
                None => {
                    println!("[routes] no route found for: {}", request.uri);
                    None
                }
            },
        }
    }

    /// Allows you to configure routes in a closure using
    /// the RouteBuilder.
    pub fn configure<F>(&mut self, f: F)
    where
        F: FnOnce(&mut RouteBuilder),
    {
        let mut builder = RouteBuilder::new(self);
        f(&mut builder);
    }
}

/// Route Builder
///
/// Use this to build a collection of routes.
/// Exposed as part of the Routes API.
pub struct RouteBuilder<'a> {
    build: &'a mut Routes,
}

impl<'a> RouteBuilder<'a> {
    pub fn new(routes: &'a mut Routes) -> Self {
        RouteBuilder { build: routes }
    }

    pub fn def(&mut self, method: &str, path: &str, handler: RouteHandler) -> &mut Self {
        let method = Method::from(method);
        let method_map = self.build.routes.entry(method).or_insert(HashMap::new());
        method_map.insert(path.to_string(), handler);
        self
    }
}
