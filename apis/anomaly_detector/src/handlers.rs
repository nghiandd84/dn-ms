use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, Path, State},
    http::StatusCode,
    Json,
};
use redis::Commands;
use std::net::SocketAddr;

use crate::{
    models::{BlockEntry, BlockListResponse, BlockRequest, FingerprintEntry, FingerprintListResponse, RegisterFingerprintRequest},
    AppState,
};

const KEY_PREFIX: &str = "anomaly";
const BLOCK_SUFFIX: &str = "blocked:manual";

fn block_key(fingerprint: &str) -> String {
    format!("{}:{}:{}", KEY_PREFIX, fingerprint, BLOCK_SUFFIX)
}

pub async fn create_block(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BlockRequest>,
) -> Result<(StatusCode, Json<BlockEntry>), (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let now = chrono_now();
    let entry = BlockEntry {
        fingerprint: req.fingerprint.clone(),
        reason: req.reason.clone(),
        duration_seconds: req.duration_seconds,
        blocked_by: req.blocked_by.clone(),
        blocked_at: now.clone(),
    };

    let value = serde_json::to_string(&entry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let key = block_key(&req.fingerprint);
    let _: () = conn.set_ex(&key, &value, req.duration_seconds)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(entry)))
}

pub async fn get_block(
    State(state): State<Arc<AppState>>,
    Path(fingerprint): Path<String>,
) -> Result<Json<BlockEntry>, (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let key = block_key(&fingerprint);
    let value: Option<String> = conn.get(&key)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match value {
        Some(v) => {
            let entry: BlockEntry = serde_json::from_str(&v)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(Json(entry))
        }
        None => Err((StatusCode::NOT_FOUND, "Block not found".to_string())),
    }
}

pub async fn delete_block(
    State(state): State<Arc<AppState>>,
    Path(fingerprint): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let key = block_key(&fingerprint);
    let _: () = conn.del(&key)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_blocks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<BlockListResponse>, (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let pattern = format!("{}:*:{}", KEY_PREFIX, BLOCK_SUFFIX);
    let keys: Vec<String> = conn.keys(&pattern)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut blocks = Vec::new();
    for key in &keys {
        let value: Option<String> = conn.get(key)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        if let Some(v) = value {
            if let Ok(entry) = serde_json::from_str::<BlockEntry>(&v) {
                blocks.push(entry);
            }
        }
    }

    let total = blocks.len();
    Ok(Json(BlockListResponse { blocks, total }))
}

fn chrono_now() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    format!("{}", duration.as_secs())
}

// --- Fingerprint handlers ---

const FP_PREFIX: &str = "anomaly:fingerprint";

fn fp_key(fingerprint: &str) -> String {
    format!("{}:{}", FP_PREFIX, fingerprint)
}

pub async fn register_fingerprint(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<RegisterFingerprintRequest>,
) -> Result<Json<FingerprintEntry>, (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let now = chrono_now();
    let ip = addr.ip().to_string();
    let key = fp_key(&req.fingerprint);

    // Check if fingerprint already exists — update last_seen and request_count
    let existing: Option<String> = conn.get(&key)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let entry = if let Some(v) = existing {
        let mut entry: FingerprintEntry = serde_json::from_str(&v)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        entry.last_seen_at = now;
        entry.request_count += 1;
        entry.ip_address = ip;
        // Check block status
        let block_key = block_key(&req.fingerprint);
        entry.is_blocked = conn.exists(&block_key).unwrap_or(false);
        entry
    } else {
        let block_key = block_key(&req.fingerprint);
        let is_blocked: bool = conn.exists(&block_key).unwrap_or(false);
        FingerprintEntry {
            fingerprint: req.fingerprint.clone(),
            user_agent: req.user_agent,
            language: req.language,
            platform: req.platform,
            screen_resolution: req.screen_resolution,
            timezone: req.timezone,
            ip_address: ip,
            created_at: now.clone(),
            last_seen_at: now,
            request_count: 1,
            is_blocked,
        }
    };

    let value = serde_json::to_string(&entry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let _: () = conn.set(&key, &value)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(entry))
}

pub async fn get_fingerprint(
    State(state): State<Arc<AppState>>,
    Path(fingerprint): Path<String>,
) -> Result<Json<FingerprintEntry>, (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let key = fp_key(&fingerprint);
    let value: Option<String> = conn.get(&key)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match value {
        Some(v) => {
            let mut entry: FingerprintEntry = serde_json::from_str(&v)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            // Refresh block status
            let bk = block_key(&fingerprint);
            entry.is_blocked = conn.exists(&bk).unwrap_or(false);
            Ok(Json(entry))
        }
        None => Err((StatusCode::NOT_FOUND, "Fingerprint not found".to_string())),
    }
}

pub async fn list_fingerprints(
    State(state): State<Arc<AppState>>,
) -> Result<Json<FingerprintListResponse>, (StatusCode, String)> {
    let mut conn = state.redis.get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let pattern = format!("{}:*", FP_PREFIX);
    let keys: Vec<String> = conn.keys(&pattern)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut fingerprints = Vec::new();
    for key in &keys {
        let value: Option<String> = conn.get(key)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        if let Some(v) = value {
            if let Ok(mut entry) = serde_json::from_str::<FingerprintEntry>(&v) {
                let bk = block_key(&entry.fingerprint);
                entry.is_blocked = conn.exists(&bk).unwrap_or(false);
                fingerprints.push(entry);
            }
        }
    }

    let total = fingerprints.len();
    Ok(Json(FingerprintListResponse { fingerprints, total }))
}
