use std::sync::Arc;
use crate::{
    models::{db::get_data_table::AllCallsClient, server::RequestServer},
    server::server::AppState,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use tokio::time::Instant;

//? Звонки и писма */
pub async fn handler_calls_email(
    State(data): State<Arc<AppState>>,
    opt: Option<Query<RequestServer>>,
) -> impl IntoResponse {
    let start_time_request: Instant = Instant::now();

    let start_time: String = opt.as_ref().unwrap().date_start.clone();
    let end_time: String = opt.as_ref().unwrap().date_end.clone();

    let resp = match AllCallsClient::get_calls(start_time, end_time, data.db.clone()).await {
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
