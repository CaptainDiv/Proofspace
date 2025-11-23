use anyhow::Result;
use axum::{routing::get, routing::post, Router};
use fastcrypto::{ed25519::Ed25519KeyPair, traits::KeyPair};
use nautilus_server::common::{get_attestation, health_check};
use nautilus_server::AppState;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;


mod apps {
pub mod content_attestor;
}


#[tokio::main]
async fn main() -> Result<()> {
    let eph_kp = Ed25519KeyPair::generate(&mut rand::thread_rng());


    #[cfg(not(feature = "seal-example"))]
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");


    #[cfg(feature = "seal-example")]
    let api_key = String::new();


    let state = Arc::new(AppState { eph_kp, api_key });


    #[cfg(feature = "seal-example")]
    {
        nautilus_server::app::spawn_host_init_server(state.clone()).await?;
    }


    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);


    let app = Router::new()
        .route("/", get(ping))
        .route("/get_attestation", get(get_attestation))
        .route("/process_data", post(apps::content_attestor::process_data))
        .route("/health_check", get(health_check))
        .route("/content_attestor", get(apps::content_attestor::handler))
        .with_state(state)
        .layer(cors);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))
}


async fn ping() -> &'static str {
    "Pong!"
}