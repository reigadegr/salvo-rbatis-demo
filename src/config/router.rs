use crate::controller::users_controller::{login, login_post};
use salvo::Router;
pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/login").get(login))
        .push(Router::with_path("/users/login").post(login_post))
        .push(Router::with_path("/login").post(login_post))
}
