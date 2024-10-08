use crate::auth::jwt::Claims;
use crate::extractors::workdir::TempDirHandle;
use crate::server::AppState;
use crate::threadpool::CairoVersionedInput;
use crate::utils::job::create_job;
use axum::Json;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use common::cairo_prover_input::CairoProverInput;
use serde_json::json;

pub async fn root(
    State(app_state): State<AppState>,
    TempDirHandle(path): TempDirHandle,
    _claims: Claims,
    Json(program_input): Json<CairoProverInput>,
) -> impl IntoResponse {
    let thread_pool = app_state.thread_pool.clone();
    let job_store = app_state.job_store.clone();
    let job_id = create_job(&job_store).await;
    let thread = thread_pool.lock().await;
    thread
        .execute(
            job_id,
            job_store,
            path,
            CairoVersionedInput::Cairo(program_input),
        )
        .await
        .into_response();
    let body = json!({
        "job_id": job_id
    });
    (StatusCode::ACCEPTED, body.to_string())
}
