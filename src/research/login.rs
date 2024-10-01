use salvo::{handler, Request, Response};
use crate::{RB, Users};

impl_select!(Users{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
impl_select!(Users{select_by_type(phone:String) -> Option => "`where phone = #{phone} limit 1`"});
#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let uid = req.query::<i8>("id").unwrap();
    let data = Users::select_by_id(&mut RB.clone(), uid.to_string()).await.unwrap();
    println!("{:?}", data);
    res.render(serde_json::to_string(&data).unwrap());
}

#[handler]
pub async fn get_user2(req: &mut Request, res: &mut Response) {
    let uid = req.query::<&str>("phone").unwrap();
    let data = Users::select_by_type(&mut RB.clone(), uid.to_string()).await.unwrap();
    println!("{:?}", data);
    res.render(serde_json::to_string(&data).unwrap());
}
