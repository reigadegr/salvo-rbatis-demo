use salvo::Router;
use crate::research::login::get_user;
use crate::research::login::get_user2;
use crate::research::login::user_login;

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/users").get(get_user))
        .push(Router::with_path("/login").get(user_login))
}
