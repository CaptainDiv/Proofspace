use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use blake3;

use nautilus_server::common::{
    to_signed_response,
    IntentMessage,
    IntentScope,
    ProcessDataRequest,
    ProcessedDataResponse,
};

use nautilus_server::{AppState, EnclaveError};

// Intent code 
pub const PROOF_INTENT: u8 = 42;  

// Client request payload
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentRequest {
    pub content: String,
}

// Response payload 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentResponse {
    pub content_hash: String,
}

// /process_data handler used by the enclosing server
pub async fn process_data(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ProcessDataRequest<ContentRequest>>,
) -> Result<Json<ProcessedDataResponse<IntentMessage<ContentResponse>>>, EnclaveError> {
    let hash = blake3::hash(request.payload.content.as_bytes())
        .to_hex()
        .to_string();

    let timestamp_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| EnclaveError::GenericError(format!("failed to get timestamp: {e}")))?
        .as_millis() as u64;

    Ok(Json(to_signed_response(
        &state.eph_kp,
        ContentResponse { content_hash: hash },
        timestamp_ms,
        IntentScope::ProcessData,
    )))
}

// Simple HTTP handler to sanity-check the enclave process
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