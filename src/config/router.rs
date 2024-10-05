use salvo::Router;
use crate::controller::login::users_login;

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/login").get(users_login))
}
