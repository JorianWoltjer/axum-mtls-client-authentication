// TODO: clean up deps
use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError,
};
use rustls::{server::AllowAnyAnonymousOrAuthenticatedClient, ServerConfig};
use rustls::{Certificate, RootCertStore};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::{middleware::AddExtension, routing::get, Extension, Router};
use axum_server::{
    accept::Accept,
    tls_rustls::{RustlsAcceptor, RustlsConfig},
};
use futures_util::future::BoxFuture;
use std::{io, net::SocketAddr, sync::Arc};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::server::TlsStream;
use tower::Layer;
use x509_parser::prelude::{FromDer, X509Certificate};

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[derive(Debug, Clone)]
struct TlsData {
    hostname: Option<Arc<str>>,
    peer_certificates: Option<Vec<Certificate>>,
}

#[derive(Debug, Clone)]
struct CustomAcceptor {
    inner: RustlsAcceptor,
}

impl CustomAcceptor {
    fn new(inner: RustlsAcceptor) -> Self {
        Self { inner }
    }
}

impl<I, S> Accept<I, S> for CustomAcceptor
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: Send + 'static,
{
    type Stream = TlsStream<I>;
    type Service = AddExtension<S, TlsData>;
    type Future = BoxFuture<'static, io::Result<(Self::Stream, Self::Service)>>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        let acceptor = self.inner.clone();

        Box::pin(async move {
            let (stream, service) = acceptor.accept(stream, service).await?;
            let server_conn = stream.get_ref().1;
            let sni_hostname = TlsData {
                hostname: server_conn.server_name().map(From::from),
                peer_certificates: server_conn.peer_certificates().map(From::from),
            };
            let service = Extension(sni_hostname).layer(service);

            Ok((stream, service))
        })
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tls_rustls=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ports = Ports {
        http: 8000,
        https: 8443,
    };
    // optional: spawn a second server to redirect http requests to this server
    tokio::spawn(redirect_http_to_https(ports));

    // configure certificate and private key used by https
    let mut store = RootCertStore::empty();
    let ca_certs = load_certificates_from_pem("certs/ca-cert.pem").unwrap();
    for cert in &ca_certs {
        store.add(cert).unwrap();
    }
    let client_cert_verifier = Arc::new(AllowAnyAnonymousOrAuthenticatedClient::new(store));
    // let client_cert_verifier = Arc::new(AllowAnyAuthenticatedClient::new(store));
    let private_key = rustls::PrivateKey(fs::read("certs/server-key.der").unwrap());
    let certs = load_certificates_from_pem("certs/server-cert.pem").unwrap();

    let config = RustlsConfig::from_config(Arc::new(
        ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(client_cert_verifier)
            // .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .unwrap(),
    ));

    // TODO: make middleware
    let app = Router::new()
        .route("/", get(handler))
        .route("/test", get(test));

    // run https server
    let addr = SocketAddr::from(([0, 0, 0, 0], ports.https));
    tracing::debug!("listening on {}", addr);
    axum_server::bind(addr)
        .acceptor(CustomAcceptor::new(RustlsAcceptor::new(config)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}

async fn test(tls_data: Extension<TlsData>) -> String {
    // TODO: error handling
    if let Some(peer_certificates) = &tls_data.peer_certificates {
        // TODO: find first signed by MyCA
        let cert = X509Certificate::from_der(&peer_certificates[0].0)
            .unwrap()
            .1;
        // TODO: find what index cn we need
        let cn = cert.subject().iter_common_name().next().unwrap();
        let cn = cn.as_str().unwrap();
        format!("Authenticated as {cn:?}")
    } else {
        format!("{tls_data:?}")
    }
}

async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}

fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader).map(|c| c.unwrap().to_vec());

    Ok(certs.map(Certificate).collect())
}
