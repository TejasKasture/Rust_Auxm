use axum::Json;
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize,Debug)]

pub struct Mirrorjson{
    message:String,
}
#[derive(Serialize)]
pub struct MirrorjsonRespose{
    message:String,
    Response:String,
}

pub async fn mirror_body_json(Json(body):Json<Mirrorjson>) -> Json<MirrorjsonRespose>
{
      Json(MirrorjsonRespose { message: body.message, Response: "HEllo Axum ...".to_owned() })
}