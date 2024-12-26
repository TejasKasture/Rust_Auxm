use axum::Extension;

use super::SharedData;

pub async fn middlelayer_message(Extension(shared_data):Extension<SharedData>)->String
{
        shared_data.message.to_owned()
}