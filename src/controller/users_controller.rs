use crate::services::r#impl::users_services_impl::UsersServicesImpl;
use salvo::{handler, Request, Response};

use crate::services::users_service::UsersService;

//示例：http://127.0.0.1:5800/login/?username=admin&password=123456

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    <UsersServicesImpl as UsersService>::login(req, res).await
}

#[handler]
pub async fn login_post(req: &mut Request, res: &mut Response) {
    <UsersServicesImpl as UsersService>::login_post(req, res).await
}

#[handler]
pub async fn hello(req: &mut Request) -> &'static str {
    println!("打印helloworld");
    println!("请求体：{:?}", req);
    "Hello World"
}

#[handler]
pub async fn get_list(req: &mut Request, res: &mut Response){
    <UsersServicesImpl as UsersService>::get_list(req, res).await
}
