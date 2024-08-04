use std::sync::Arc;

use axum::{middleware, routing::get, Extension, Router};
use mtls_client_authentication::{
    auth::{auth_middleware, Auth},
    load_store_from_pem, AppState,
};
use rustls::server::AllowAnyAuthenticatedClient;

const HOST: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    let store = load_store_from_pem("certs/ca-cert.pem").unwrap();
    let client_cert_verifier = AllowAnyAuthenticatedClient::new(store);
    let state = AppState {
        verifier: Arc::new(client_cert_verifier),
    };

    // Routes
    let app = Router::new()
        .route("/", get(get_index))
        .merge(
            Router::new() // Authenticated routes
                .route("/auth", get(get_auth))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .with_state(state);

    // Start server
    println!("Listening on {}...", HOST);
    axum_server::bind(HOST.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_index() -> &'static str {
    "Hello, world!"
}

async fn get_auth(Extension(auth): Extension<Auth>) -> String {
    format!("Authenticated as {:?}", auth.username)
}
