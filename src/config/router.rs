use crate::controller::users_controller::users_login;
use salvo::Router;

pub fn init_router() -> Router {
    Router::new().push(Router::with_path("/login").get(users_login))
}
