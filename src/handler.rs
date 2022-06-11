use crate::{WebResult};
use warp::Reply;

pub async fn get() -> WebResult<impl Reply> {
    Ok("Hola info")
}