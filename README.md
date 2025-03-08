# HTTP/3 Client Project

This project demonstrates a simple HTTP/3 client using the `quinn` and `h3` libraries in Rust. The client establishes a QUIC connection to a server, sends a JSON payload, and receives a response. The project is designed to showcase how to use HTTP/3 over QUIC for secure and efficient communication.

![HTTP/3 Client Flow](https://upload.wikimedia.org/wikipedia/commons/thumb/6/6b/QUIC_Transport_Layer.svg/1200px-QUIC_Transport_Layer.svg.png)
https://aws.amazon.com/blogs/aws/new-http-3-support-for-amazon-cloudfront/

## Features

- **QUIC Protocol**: Utilizes the QUIC transport protocol for low-latency, secure communication.
- **HTTP/3 Support**: Implements HTTP/3 using the `h3` library, which is built on top of QUIC.
- **TLS Encryption**: Secures the connection using TLS 1.3 with custom CA certificates.
- **JSON Payload**: Sends a JSON payload to the server as part of the HTTP/3 request.
- **Asynchronous I/O**: Leverages Tokio for asynchronous networking and I/O operations.

## Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
- **OpenSSL**: Required for TLS support. Install it via your package manager or from [OpenSSL's website](https://www.openssl.org/).

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/http3-client.git
cd http3-client
```

### 2. Build the Project

```bash
cargo build
```

### 3. Run the Client

```bash
cargo run
```

The client will attempt to connect to `https://127.0.0.1:8080` and send a JSON payload. Ensure that your server is running and configured to accept HTTP/3 connections.

## Data Flow Diagram

Below is a simple representation of how the client sends data to the server over HTTP/3:

![HTTP/3 Request Flow](https://upload.wikimedia.org/wikipedia/commons/2/2b/HTTP_3_request_response.svg)

## Configuration

### CA Certificates

The client requires a CA certificate to establish a secure TLS connection. Place your CA certificate in the `certs/ca.crt` file. The `load_ca_cert` function reads this file and configures the TLS client.

### Customizing the Request

You can modify the `UserRegistration` struct and the JSON payload in the `main` function to suit your needs. The current implementation sends a simple JSON object with an address and a public key.

```rust
let user_registration = UserRegistration {
    address: "127.0.0.1".to_string(),
    public_key: vec![8], // Example public key as Vec<u8>
};
```

### Changing the Server Address

To connect to a different server, modify the `uri` variable in the `main` function:

```rust
let uri: http::Uri = "https://your.server.address".parse()?;
```

## Dependencies

- `quinn`: A QUIC implementation in Rust.
- `h3`: An HTTP/3 library built on top of QUIC.
- `rustls`: A modern TLS library in Rust.
- `tokio`: An asynchronous runtime for Rust.
- `serde_json`: For JSON serialization and deserialization.
- `tracing`: For logging and diagnostics.

## Example Output

When running the client, you should see output similar to the following:

```
INFO  [http3_client] DNS lookup for "https://127.0.0.1": 127.0.0.1:8080
INFO  [http3_client] DNS client configured: ClientConfig { ... }
INFO  [http3_client] QUIC connection established
INFO  [http3_client] sending request ...
INFO  [http3_client] receiving response ...
INFO  [http3_client] response: HTTP/3.0 200 OK
INFO  [http3_client] headers: { ... }
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

- The `quinn` and `h3` teams for their excellent libraries.
- The Rust community for providing a robust ecosystem for systems programming.

## Further Reading

- [QUIC Protocol](https://en.wikipedia.org/wiki/QUIC)
- [HTTP/3 Specification](https://www.rfc-editor.org/rfc/rfc9114.html)
- [Rust Programming Language](https://www.rust-lang.org/)

