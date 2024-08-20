use crate::{
    models::{ db::get_data_table::ClientCalibri, server::StatusClientList},
    server::server::AppState,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::time::Instant;

//? Список активных клиентов  */
pub async fn handler_status_client(
    State(data): State<Arc<AppState>>,
    opt: Option<Query<StatusClientList>>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let status_client: &String = &opt.unwrap().status;

    let resp = match ClientCalibri::get_all_clients_status(data.db.clone(), &status_client).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "err": err.to_string(),
            })),
        ),
    };

    resp
}

