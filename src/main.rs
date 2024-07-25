use axum::{middleware, routing::get, Extension, Router};
use axum_server::tls_rustls::{RustlsAcceptor, RustlsConfig};
use mtls_client_authentication::auth::{auth_middleware, Auth};
use rustls::{server::AllowAnyAnonymousOrAuthenticatedClient, ServerConfig};
use std::sync::Arc;

use mtls_client_authentication::{
    auth::TLSAcceptor, load_certificates_from_pem, load_private_key_from_pem, load_store_from_pem,
};

const HOST: &str = "0.0.0.0:8443";

#[tokio::main]
async fn main() {
    // Set up TLS
    let store = load_store_from_pem("certs/ca-cert.pem").unwrap();
    let client_cert_verifier: Arc<AllowAnyAnonymousOrAuthenticatedClient> = Arc::new(AllowAnyAnonymousOrAuthenticatedClient::new(store));
    let private_key = load_private_key_from_pem("certs/server-key.pem").unwrap();
    let certs = load_certificates_from_pem("certs/server-cert.pem").unwrap();

    let config = RustlsConfig::from_config(Arc::new(
        ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(client_cert_verifier)
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
    axum_server::bind(HOST.parse().unwrap())
        .acceptor(TLSAcceptor::new(RustlsAcceptor::new(config)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_index() -> &'static str {
    "Hello, World!"
}

async fn get_auth(Extension(auth): Extension<Auth>) -> String {
    format!("Authenticated as {:?}", auth.username)
}
