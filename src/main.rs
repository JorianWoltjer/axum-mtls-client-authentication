use std::sync::Arc;

use axum::{middleware, routing::get, Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use mtls_client_authentication::{
    auth::{auth_middleware, Auth},
    load_certificates_from_pem, load_private_key_from_pem, load_store_from_pem,
};
use rustls::{server::AllowAnyAuthenticatedClient, ServerConfig};

const HOST: &str = "0.0.0.0:8443";

#[tokio::main]
async fn main() {
    let store = load_store_from_pem("certs/backend/ca-cert.pem").unwrap();
    let client_cert_verifier = AllowAnyAuthenticatedClient::new(store);
    let private_key = load_private_key_from_pem("certs/backend/server-key.pem").unwrap();
    let certs = load_certificates_from_pem("certs/backend/server-cert.pem").unwrap();

    let config = RustlsConfig::from_config(Arc::new(
        ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(Arc::new(client_cert_verifier))
            .with_single_cert(certs, private_key)
            .unwrap(),
    ));

    // Routes
    let app = Router::new().route("/", get(get_index)).merge(
        Router::new() // Authenticated routes
            .route("/auth", get(get_auth))
            .route_layer(middleware::from_fn(auth_middleware)),
    );

    // Start server
    println!("Listening on {}...", HOST);
    axum_server::bind_rustls(HOST.parse().unwrap(), config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_index() -> &'static str {
    "Hello, world!"
}

async fn get_auth(Extension(auth): Extension<Auth>) -> String {
    format!(
        "Authenticated as {:?} (serial: {:#x})",
        auth.username, auth.serial
    )
}
