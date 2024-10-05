use crate::services::Impl::users_services_impl::UsersServicesImpl;
use salvo::{handler, Request, Response};

use crate::services:: users_service:: UsersService;

//示例：http://127.0.0.1:5800/login/?username=admin&password=123456

#[handler]
pub async fn users_login(req: &mut Request, res: &mut Response) -> () {
    return <UsersServicesImpl as UsersService>::login(req, res).await;
}
