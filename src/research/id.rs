use salvo::{handler, Request, Response};
use crate::{RB, Users};
use crate::res::result::ResponseData;

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let uid = req.query::<i8>("id").unwrap();
    let data = Users::select_by_id(&mut RB.clone(), uid.to_string()).await.unwrap();
    let data = ResponseData::success(data, "查询成功");
    println!("{:?}", data);
    res.render(serde_json::to_string(&data).unwrap());
}
