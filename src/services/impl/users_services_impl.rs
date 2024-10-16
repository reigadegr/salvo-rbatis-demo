use crate::config::init::RB;
use crate::config::redis::{redis_delete, redis_read, redis_write};
use crate::pojo::token::Token;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;
use crate::services::users_service::UsersService;
use salvo::prelude::*;
use salvo::{Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Extractible, Clone)]
struct UserInfo {
    username: String,
    password: String,
}
pub struct UsersServicesImpl;
impl UsersService for UsersServicesImpl {
    async fn login_post(req: &mut Request, res: &mut Response) {
        //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
        println!("这是login_post。请求体：{:?}", req);
        let user_info = req.parse_json::<UserInfo>().await;
        println!("{:?}", &user_info);
        let user_info = user_info.unwrap();
        let username = user_info.username;
        let password = user_info.password;
        let data = Box::new(
            Users::login(&RB.clone(), username.to_string(), password.to_string())
                .await
                .unwrap(),
        );

        if data.is_none() {
            let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }

        let rs = redis_write(
            "now_user_role",
            &<Option<Users> as Clone>::clone(&data).unwrap()._type,
        )
        .await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            let data: ResponseData<()> = ResponseData::error("Redis连接错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }

        let rs = redis_write(
            "now_user_name",
            &<Option<Users> as Clone>::clone(&data).unwrap().username,
        )
        .await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            let data: ResponseData<()> = ResponseData::error("Redis连接错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let now_token = Token {
            token: format!(
                "token-{}",
                &*<Option<Users> as Clone>::clone(&data).unwrap()._type
            ),
        };
        let data = ResponseData::success(now_token, "登录成功");

        println!("{:?}", &data);
        res.render(serde_json::to_string(&data).unwrap())
    }

    async fn users_info(res: &mut Response) {
        let roles = redis_read("now_user_role").await.unwrap_or_default();
        let roles = vec![roles];
        let username = redis_read("now_user_name").await.unwrap_or_default();
        let rs_data = UserData {
            username,
            roles: roles.clone(),
        };
        let data =
            ResponseData::success(rs_data, &("获取".to_owned() + &roles[0] + "类型用户成功"));
        res.render(serde_json::to_string(&data).unwrap());
    }

    async fn users_logout(res: &mut Response) {
        let rs = redis_delete("now_user_role").await;
        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("Redis链接有误");
            res.render(serde_json::to_string(&data).unwrap())
        }
        let rs = redis_delete("now_user_name").await;
        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("Redis链接有误");
            res.render(serde_json::to_string(&data).unwrap())
        }
        let rs = ResponseData::success("", "清除Redis缓存成功");
        res.render(serde_json::to_string(&rs).unwrap())
    }
}

// 确保这个结构体实现了 Serialize 特性
#[derive(Serialize, Default, Deserialize)]
struct UserData {
    username: String,
    roles: Vec<String>,
}
