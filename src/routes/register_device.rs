use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    core::errors::http_errors::internal_error,
    models::device::{CreateDevice, Device},
    schema::devices,
    types::db::Pool,
};

pub async fn register_device(
    State(pool): State<Pool>,
    Json(new_device): Json<CreateDevice>,
) -> Result<Json<Device>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let res = diesel::insert_into(devices::table)
        .values(new_device)
        .returning(Device::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}
