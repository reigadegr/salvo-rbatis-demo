use crate::config::init::RB;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;
use crate::services::users_service::UsersService;
use salvo::prelude::*;
use salvo::{Request, Response};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Extractible, Clone)]
struct UserInfo {
    pub username: String,
    pub password: String,
}
pub struct UsersServicesImpl;
impl UsersService for UsersServicesImpl {
    async fn login(req: &mut Request, res: &mut Response) {
        //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
        println!("{:?}", req);
        let username = req.query::<&str>("username").unwrap();
        let password = req.query::<&str>("password").unwrap();
        let data = Users::login(&RB.clone(), username.to_string(), password.to_string())
            .await
            .unwrap();
        if data.is_none() {
            let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
            println!("{:?}", data);
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let data = ResponseData::success(data, "登录成功");
        println!("{:?}", data);
        res.render(serde_json::to_string(&data).unwrap())
    }

    async fn login_post(req: &mut Request, res: &mut Response) {
        //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
        println!("请求体：{:?}", req);
        let user_info = req.parse_json::<UserInfo>().await;
        println!("{:?}", &user_info);
        let a2 = user_info.unwrap();
        let username = a2.username;
        let password = a2.password;
        let data = ResponseData::success("data", "登录成功");
        println!("{:?}", data);
        let data = Users::login(&RB.clone(), username.to_string(), password.to_string())
            .await
            .unwrap();
        if data.is_none() {
            let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
            println!("{:?}", data);
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let data = ResponseData::success(data, "登录成功");
        println!("{:?}", data);
        res.render(serde_json::to_string(&data).unwrap())
    }
}
