use salvo::Router;
use crate::controller::users_controller::users_login;

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/login").get(users_login))
}
