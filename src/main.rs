use bytes::Bytes;
use futures::future;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Debug;
use std::{error::Error, sync::Arc};
use tokio::io::AsyncWriteExt;
use tracing::info;
mod utils;
use utils::{configure_tls, load_ca_cert};

#[derive(Serialize, Deserialize, Debug)]
struct UserRegistration {
    address: String,
    public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserAddress {
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    let uri: http::Uri = "https://127.0.0.1".parse()?;
    if uri.scheme() != Some(&http::uri::Scheme::HTTPS) {
        Err("uri scheme must be 'https'")?;
    }

    let auth = uri.authority().ok_or("Uri must have host")?.clone();
    let port = auth.port_u16().unwrap_or(8080);
    let addr = tokio::net::lookup_host((auth.host(), port))
        .await?
        .next()
        .ok_or("dns found no addresses")?;
    info!("DNS lookup for {:?}: {:?}", uri, addr);

    // Load CA certificates
    let roots = load_ca_cert("certs/ca.crt")?;
    let client_crypto = configure_tls(roots)?;
    //info!("DNS client configured: {:?}", client_crypto);

    let mut client_endpoint = h3_quinn::quinn::Endpoint::client("0.0.0.0:0".parse()?)?;
    let client_config = quinn::ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)?,
    ));

    client_endpoint.set_default_client_config(client_config);
    let conn = client_endpoint.connect(addr, auth.host())?.await?;
    info!("QUIC connection established");

    // create h3 client

    // h3 is designed to work with different QUIC implementations via
    // a generic interface, that is, the [`quic::Connection`] trait.
    // h3_quinn implements the trait w/ quinn to make it work with h3.
    let quinn_conn = h3_quinn::Connection::new(conn);
    let (mut driver, mut send_request) = h3::client::new(quinn_conn).await?;

    let drive = async move {
        future::poll_fn(|cx| driver.poll_close(cx)).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    // In the following block, we want to take ownership of `send_request`:
    // the connection will be closed only when all `SendRequest`s instances
    // are dropped.
    //
    //             So we "move" it.
    //                  vvvv
    let request = async move {
        info!("sending request ...");
        // Create an instance of UserRegistration
        let user_registration = UserRegistration {
            address: "127.0.0.1".to_string(),
            public_key: vec![8], // Example public key as Vec<u8>
        };

        let user_addr = UserAddress {
            address: "127.0.0.1".to_string(),
        };

        // Serialize the struct to JSON
        // let body = json!({
        //     "address": user_registration.address,
        //     "public_key": user_registration.public_key, // Serialize Vec<u8> directly
        // })
        // .to_string();
        let body = json!({
            "address": user_addr.address,
        })
        .to_string();

        println!("Serialized JSON body: {}", body);
        //let stream = post(send_request, uri, body);

        let req = http::Request::builder()
            .uri("https://127.0.0.1/getAddress")
            .header("Content_Type", "application/json")
            .method("POST")
            .body(())?;

        // sending request results in a bidirectional stream,
        // which is also used for receiving response
        let mut stream = send_request.send_request(req).await?;
        stream.send_data(Bytes::from(body)).await?;

        // finish on the sending side
        stream.finish().await?;

        info!("receiving response ...");

        let resp = stream.recv_response().await?;

        info!("response: {:?} {}", resp.version(), resp.status());
        info!("headers: {:#?}", resp.headers());

        // `recv_data()` must be called after `recv_response()` for
        // receiving potential response body
        while let Some(mut chunk) = stream.recv_data().await? {
            let mut out = tokio::io::stdout();
            out.write_all_buf(&mut chunk).await?;
            out.flush().await?;
        }

        Ok::<_, Box<dyn std::error::Error>>(())
    };

    let (req_res, drive_res) = tokio::join!(request, drive);
    req_res?;
    drive_res?;

    // wait for the connection to be closed before exiting
    client_endpoint.wait_idle().await;

    Ok(())
}
