use salvo::{handler, Request, Response};
use crate::services::users_services::UsersServices;

//示例：http://127.0.0.1:5800/login/?username=admin&password=123456

#[handler]
pub async fn users_login(req: &mut Request, res: &mut Response) -> () {
    return UsersServices::login(req, res).await;
}

