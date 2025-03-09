use bytes::Bytes;
use h3::{client::SendRequest, quic::OpenStreams};
use http::Uri;
use std::error::Error;

// pub async fn post(
//     mut send_request: SendRequest<OpenStreams, Bytes>,
//     uri: Uri,
//     body: String,
// ) -> Result<(), Box<dyn Error>> {
//     let req = http::Request::builder()
//         .uri(uri)
//         .header("Content_Type", "application/json")
//         .method("POST")
//         .body(())?;
//     // sending request results in a bidirectional stream,
//     // which is also used for receiving response
//     let mut stream = send_request.send_request(req).await?;
//     stream.send_data(Bytes::from(body)).await?;
//
//     // finish on the sending side
//     stream.finish().await?;
//
//     Ok(stream)
//}
