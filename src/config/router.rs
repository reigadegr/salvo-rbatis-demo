use crate::controller::users_controller::{hello, login, login_post, get_list};
use salvo::Router;
pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("").get(hello))
        .push(Router::with_path("/login").get(login))
        .push(Router::with_path("/users/login").post(login_post))
        .push(Router::with_path("/login").post(login_post))
        .push(Router::with_path("/gl").get(get_list))
}
