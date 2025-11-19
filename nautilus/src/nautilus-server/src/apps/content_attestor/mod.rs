use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::{
    to_signed_response, IntentMessage, IntentScope, ProcessDataRequest, ProcessedDataResponse,
};
use crate::{AppState, EnclaveError};

// Intent must match your Move contract constant.
pub const CONTENT_INTENT: U8 = 42;

// Client request payload
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentRequest {
    pub content_hash: String,
}

// /process_data
pub async fn process_data(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ProcessDataRequest<ContentRequest>>,
) -> Result<Json<ProcessedDataResponse<IntentMessage<ContentResponse>>>, EnclaveError> {
    let hash = blake3::hash(request.payload.content.as_bytes())
    .to_hex()
    .to_string();

    let timestamp_ms = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .map_err(|e| EnclaveError::GenericError(format!("failed to get timestapp: {e}")))?
    .as_millis() as u64;

    Ok(Json(to_signed_response(
        &state.eph_kp,
        ContentResponse { content_hash: hash },
        timestamp_ms,
        IntentScope::ProcessData,
    )))
}

pub async fn handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "content_attestor enclave is running"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use fastcrypto::{ed25519::Ed25519KeyPair, traits::KeyPair};

    #[tokio::test]
    async fn test_process_data_returns_hash() {
        let state = Arc::new(AppState {
            eph_kp: Ed25519KeyPair::generate(&mut rand::thread_rng()),
            api_key: String::new(),
        });

        let resp = process_data(
            State(state),
            Json(ProcessDataRequest {
                payload: ContentRequest {
                    content: "hello lagos".to_string(),
                },
            }),
        )
        .await
        .unwrap();

        let expected = blake3::hash("hello lagos".as_bytes()).to_hex().to_string();
        assert_eq!(resp.response.data.content_hash, expected);
        assert!(resp.signature.len() > 0);
        assert!(resp.response.timestamp_ms > 0);
    }
}