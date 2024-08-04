use rustls::{server::ClientCertVerifier, Certificate, RootCertStore};
use std::{fs::File, io::BufReader, sync::Arc};

pub mod auth;

#[derive(Clone, Debug)]
pub struct AppState {
    pub verifier: Arc<dyn ClientCertVerifier>,
}

pub fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader).map(|c| c.unwrap().to_vec());

    Ok(certs.map(Certificate).collect())
}

pub fn load_store_from_pem(path: &str) -> std::io::Result<RootCertStore> {
    let ca_certs = load_certificates_from_pem(path)?;
    let mut store = RootCertStore::empty();
    for cert in &ca_certs {
        store.add(cert).unwrap();
    }

    Ok(store)
}
