use salvo::{Request, Response};
pub mod Impl;
// 定义 UserService trait 作为接口
pub trait UsersService {
    async fn login(req: &mut Request, res: &mut Response);
}
