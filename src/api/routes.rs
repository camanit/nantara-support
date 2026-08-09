use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::ai::client::{analyze_system_logs, analyze_user_query};
use crate::diagnostics::{
    logs::get_recent_system_logs,
    network::{fix_network, run_network_check},
    process::{get_top_processes, kill_process, ProcessItem},
    spooler::fix_printer_spooler,
    system::{get_system_metrics, run_dism_repair, run_gpupdate, run_sfc_scan, SystemMetrics},
};
use crate::license::verifier::{generate_pro_key, load_local_license, parse_and_verify_license};

#[derive(Serialize)]
pub struct SystemStatusResponse {
    pub metrics: SystemMetrics,
    pub top_processes: Vec<ProcessItem>,
}

#[derive(Deserialize)]
pub struct KillProcessRequest {
    pub pid: u32,
}

#[derive(Deserialize)]
pub struct AiChatRequest {
    pub message: String,
}

#[derive(Deserialize)]
pub struct ActivateLicenseRequest {
    pub license_key: String,
}

#[derive(Deserialize)]
pub struct GenerateLicenseRequest {
    pub client_name: String,
    pub expiry_date: String,
}

#[derive(Serialize)]
pub struct GenerateLicenseResponse {
    pub pro_key: String,
}

pub fn create_router() -> Router {
    Router::new()
        .route("/api/status", get(get_status_handler))
        .route("/api/diagnose/network", get(diagnose_network_handler))
        .route("/api/diagnose/logs", get(diagnose_logs_handler))
        .route("/api/fix/network", post(fix_network_handler))
        .route("/api/fix/spooler", post(fix_spooler_handler))
        .route("/api/fix/sfc", post(fix_sfc_handler))
        .route("/api/fix/dism", post(fix_dism_handler))
        .route("/api/fix/gpupdate", post(fix_gpupdate_handler))
        .route("/api/process/kill", post(kill_process_handler))
        .route("/api/ai/chat", post(ai_chat_handler))
        .route("/api/ai/analyze-logs", post(ai_analyze_logs_handler))
        .route("/api/license/status", get(license_status_handler))
        .route("/api/license/activate", post(license_activate_handler))
        .route("/api/license/generate", post(license_generate_handler))
}

async fn get_status_handler() -> impl IntoResponse {
    let metrics = get_system_metrics();
    let top_processes = get_top_processes(10);
    Json(SystemStatusResponse {
        metrics,
        top_processes,
    })
}

async fn diagnose_network_handler() -> impl IntoResponse {
    let result = run_network_check();
    Json(result)
}

async fn diagnose_logs_handler() -> impl IntoResponse {
    let result = get_recent_system_logs();
    Json(result)
}

async fn fix_network_handler() -> impl IntoResponse {
    let result = fix_network();
    Json(result)
}

async fn fix_spooler_handler() -> impl IntoResponse {
    let result = fix_printer_spooler();
    Json(result)
}

async fn fix_sfc_handler() -> impl IntoResponse {
    let result = run_sfc_scan();
    Json(result)
}

async fn fix_dism_handler() -> impl IntoResponse {
    let result = run_dism_repair();
    Json(result)
}

async fn fix_gpupdate_handler() -> impl IntoResponse {
    let result = run_gpupdate();
    Json(result)
}

async fn kill_process_handler(Json(payload): Json<KillProcessRequest>) -> impl IntoResponse {
    let result = kill_process(payload.pid);
    Json(result)
}

async fn ai_chat_handler(Json(payload): Json<AiChatRequest>) -> impl IntoResponse {
    let result = analyze_user_query(&payload.message).await;
    Json(result)
}

async fn ai_analyze_logs_handler() -> impl IntoResponse {
    let logs = get_recent_system_logs();
    let log_text = format!(
        "Event Logs Output:\n{}\n\nMinidump count: {}",
        logs.event_logs.stdout,
        logs.minidump_files.len()
    );
    let result = analyze_system_logs(&log_text).await;
    Json(result)
}

async fn license_status_handler() -> impl IntoResponse {
    let status = load_local_license();
    Json(status)
}

async fn license_activate_handler(Json(payload): Json<ActivateLicenseRequest>) -> impl IntoResponse {
    let status = parse_and_verify_license(&payload.license_key);
    if status.is_pro {
        let _ = fs::write("license.key", payload.license_key.trim());
    }
    Json(status)
}

async fn license_generate_handler(Json(payload): Json<GenerateLicenseRequest>) -> impl IntoResponse {
    let key = generate_pro_key(&payload.client_name, &payload.expiry_date);
    Json(GenerateLicenseResponse { pro_key: key })
}
