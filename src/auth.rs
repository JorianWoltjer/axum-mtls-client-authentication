use std::time::SystemTime;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use rustls::Certificate;
use rustls_pemfile::Item;
use urlencoding::decode;
use x509_parser::prelude::{FromDer, X509Certificate};

use crate::AppState;

#[derive(Debug, Clone)]
pub struct Auth {
    pub username: String,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, &'static str> {
    // Client certificate is passed in the header by proxy
    let header_cert = request
        .headers()
        .get("X-Client-Cert")
        .map(|v| decode(v.to_str().unwrap()).unwrap());

    if let Some(s) = header_cert {
        let item = rustls_pemfile::read_one_from_slice(s.as_bytes())
            .map(|c| c.unwrap().0)
            .map_err(|_| "invalid client certificate format")?;

        match item {
            Item::X509Certificate(cert) => {
                // We verify it again (also in proxy) here to ensure the client certificate is valid
                let cert = Certificate(cert.to_vec());
                let result = state
                    .verifier
                    .verify_client_cert(&cert, &[], SystemTime::now());
                if result.is_err() {
                    return Err("invalid client certificate signature");
                }

                let cert = X509Certificate::from_der(&cert.0)
                    .map_err(|_| "invalid client certificate format")?
                    .1;
                // Extract first common name
                let cn = cert
                    .subject()
                    .iter_common_name()
                    .next()
                    .ok_or("missing common name in client certificate")?
                    .as_str()
                    .map_err(|_| "invalid common name in client certificate")?;

                // Pass it to the next handler
                request.extensions_mut().insert(Auth {
                    username: cn.to_string(),
                });
            }
            _ => return Err("invalid client certificate type"),
        }
    } else {
        return Err("missing client certificate header");
    }

    Ok(next.run(request).await)
}
