use salvo::{handler, Request, Response};
use crate::{config::init::RB, pojo::users::Users};
use crate::res::result::ResponseData;

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) -> () {
    let uid = req.query::<i8>("id").unwrap();
    let data = Users::select_by_id(&RB.clone(), uid.to_string()).await.unwrap();
    if data.is_none() {
        let data: ResponseData<()> = ResponseData::error("用户不存在");
        println!("{:?}", data);
        return res.render(serde_json::to_string(&data).unwrap());
    }
    let data = ResponseData::success(data, "查询成功");
    println!("{:?}", data);
    return res.render(serde_json::to_string(&data).unwrap());
}
