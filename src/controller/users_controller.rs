use crate::services::r#impl::users_services_impl::UsersServicesImpl;
use salvo::{handler, Request, Response};

use crate::services::users_service::UsersService;

#[handler]
pub async fn login_post(req: &mut Request, res: &mut Response) {
    <UsersServicesImpl as UsersService>::login_post(req, res).await
}

#[handler]
pub async fn users_info(res: &mut Response) {
    <UsersServicesImpl as UsersService>::users_info(res).await
}

#[handler]
pub async fn users_logout(res: &mut Response) {
    <UsersServicesImpl as UsersService>::users_logout(res).await
}

#[handler]
pub async fn hello(req: &mut Request) -> &'static str {
    println!("打印helloworld");
    println!("请求体：{:?}", req);
    "Hello World"
}
