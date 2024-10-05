use salvo::{handler, Request, Response};
use crate::{RB, Users};
use crate::res::result;
use crate::res::result::ResponseData;

impl_select!(Users{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
impl_select!(Users{login(username:String,password:String) -> Option => "`where username = #{username} AND password = #{password} limit 1`"});

#[handler]
pub async fn user_login(req: &mut Request, res: &mut Response) -> () {
    //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
    let username = req.query::<&str>("username").unwrap();
    let password = req.query::<&str>("password").unwrap();
    let data = Users::login(&mut RB.clone(), username.to_string(), password.to_string()).await.unwrap();
    if data.is_none() {
        let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
        println!("{:?}", data);
        return res.render(serde_json::to_string(&data).unwrap());
    }
    let data = ResponseData::success(data, "登录成功");
    println!("{:?}", data);
    return res.render(serde_json::to_string(&data).unwrap());
}
