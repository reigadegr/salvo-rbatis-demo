use salvo::{Request, Response};
// 定义 UserService trait 作为接口
pub trait UsersService {
    async fn login(req: &mut Request, res: &mut Response);
    async fn login_post(req: &mut Request, res: &mut Response);
}
