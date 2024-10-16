use crate::res::result::ResponseData;
use salvo::{handler, Request, Response};

#[handler]
pub async fn get_video_current_tine(req: &mut Request, res: &mut Response) {
    let time = req.param::<String>("time").unwrap();
    println!("{:?}", time);
    let data = ResponseData::success(time, "成功");
    res.render(serde_json::to_string(&data).unwrap())
}
