use crate::controller::users_controller::{hello, login_post, users_info};
use salvo::Router;
pub fn init_router() -> Router {
    Router::new().push(Router::with_path("").get(hello)).push(
        Router::new()
            .path("/users")
            .push(Router::new().path("info").get(users_info))
            .push(Router::new().path("login").post(login_post)),
    )
}
