use rustls::{Certificate, RootCertStore};
use std::{fs::File, io::BufReader};

pub mod auth;

pub fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader).map(|c| c.unwrap().to_vec());

    Ok(certs.map(Certificate).collect())
}

pub fn load_private_key_from_pem(path: &str) -> std::io::Result<rustls::PrivateKey> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let keys = rustls_pemfile::ec_private_keys(&mut reader)
        .map(|k| k.unwrap())
        .next()
        .unwrap();

    Ok(rustls::PrivateKey(keys.secret_sec1_der().to_vec()))
}

pub fn load_store_from_pem(path: &str) -> std::io::Result<RootCertStore> {
    let ca_certs = load_certificates_from_pem(path)?;
    let mut store = RootCertStore::empty();
    for cert in &ca_certs {
        store.add(cert).unwrap();
    }

    Ok(store)
}
