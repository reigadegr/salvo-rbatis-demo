use std::{format, println};
use salvo::{Request, Response};
use crate::config::init::RB;
use crate::config::redis::redis_write;
use crate::pojo::token::Token;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;

// 定义 UserService trait 作为接口
pub trait UsersService {
    async fn login(req: &mut Request, res: &mut Response);
    async fn login_post(req: &mut Request, res: &mut Response);
    async fn users_info(req: &mut Request, res: &mut Response);
    async fn get_list(req: &mut Request, res: &mut Response);
}
