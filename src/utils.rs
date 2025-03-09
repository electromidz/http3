use rustls::{ClientConfig, RootCertStore};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use tracing::error;

static ALPN: &[u8] = b"h3";

pub fn load_ca_cert(path: &str) -> Result<RootCertStore, Box<dyn std::error::Error>> {
    let mut roots = rustls::RootCertStore::empty();
    let mut pem = BufReader::new(File::open(path)?);
    let certs = rustls_pemfile::certs(&mut pem);
    for cert in certs.into_iter() {
        roots.add(cert?).map_err(|e| {
            error!("Failed to add CA certificate: {}", e);
            e
        })?;
    }
    Ok(roots)
}

pub fn configure_tls(roots: RootCertStore) -> Result<ClientConfig, Box<dyn Error>> {
    let mut config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    // Enable HTTP/3 ALPN
    config.enable_early_data = true;
    config.alpn_protocols.push(ALPN.to_vec());
    Ok(config)
}
