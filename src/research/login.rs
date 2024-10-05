use salvo::{handler, Request, Response};
use crate::{RB, Users};

impl_select!(Users{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
impl_select!(Users{login(username:String,password:String) -> Option => "`where username = #{username} AND password = #{password} limit 1`"});

#[handler]
pub async fn user_login(req: &mut Request, res: &mut Response) {
    //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
    let username = req.query::<&str>("username").unwrap();
    let password = req.query::<&str>("password").unwrap();
    let data = Users::login(&mut RB.clone(), username.to_string(),password.to_string()).await.unwrap();
    println!("{:?}", data);
    res.render(serde_json::to_string(&data).unwrap());
}
#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let uid = req.query::<i8>("id").unwrap();
    let data = Users::select_by_id(&mut RB.clone(), uid.to_string()).await.unwrap();
    println!("{:?}", data);
    res.render(serde_json::to_string(&data).unwrap());
}
