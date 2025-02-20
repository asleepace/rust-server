/// HTTP Method
///
/// Allow conversion from `&str` to Method
///
/// ```
/// fn route(method: impl Into<RouteMethod>) {
///    let method = method.into();
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    Custom(String),
}

impl Method {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::Custom(s),
        }
    }
}

impl From<&String> for Method {
    fn from(s: &String) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::Custom(s.to_string()),
        }
    }
}
