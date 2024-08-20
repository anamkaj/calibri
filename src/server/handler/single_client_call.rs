use crate::{
    models::{db::get_data_table::AllCallsClient, server::RequestServerOneClient},
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

//? Получение звонков по одному клиенту */
pub async fn handler_calls_email_one_client(
    State(data): State<Arc<AppState>>,
    opt: Option<Query<RequestServerOneClient>>,
) -> impl IntoResponse {
    let start_time_request: Instant = Instant::now();

    let start_time: String = opt.as_ref().unwrap().date_start.clone();
    let end_time: String = opt.as_ref().unwrap().date_end.clone();
    let id: i64 = opt.as_ref().unwrap().id.clone();

    let resp = match AllCallsClient::get_one_calls(start_time, end_time, id, data.db.clone()).await
    {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "err": err.to_string(),
            })),
        ),
    };

    resp
}
