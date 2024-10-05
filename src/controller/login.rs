use salvo::{handler, Request, Response};
use crate::{config::init::RB, pojo::users::Users};
use crate::res::result::ResponseData;
use crate::services::users_services::users_services;

//示例：http://127.0.0.1:5800/login/?username=admin&password=123456

#[handler]
pub async fn users_login(req: &mut Request, res: &mut Response) -> () {
    return users_services::login(req, res).await;
}

